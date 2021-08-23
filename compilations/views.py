
import json
import logging
from base64 import b64encode, b64decode
from importlib.util import spec_from_file_location, module_from_spec

from django.contrib.auth.decorators import login_required
from django.conf import settings
from django.shortcuts import render, redirect
from django.urls import reverse
from rest_framework import viewsets
from rest_framework.response import Response
from rest_framework.authentication import SessionAuthentication
from rest_framework.permissions import IsAuthenticated
from requests_oauthlib import OAuth2Session
import requests

filters_location = getattr(settings, 'COMPILATIONS_FILTERS_LOCATION')
filters_spec = spec_from_file_location(
    'compilations.filters', filters_location)
filters = module_from_spec(filters_spec)
filters_spec.loader.exec_module(filters)

AUTH_BASE_URL = "https://www.reddit.com/api/v1"
AUTH_ENDPOINT = AUTH_BASE_URL + "/authorize"
TOKEN_ENDPOINT = AUTH_BASE_URL + "/access_token"
USER_AGENT = "edtwardy-savedapi/1.0;Ethan D. Twardy <ethan.twardy@gmail.com>"
REDDIT_BASE = "https://oauth.reddit.com"

@login_required
def index(request):
    """Render the app main page"""
    return render(request, 'compilations/index.html')

@login_required
def login(request):
    """Log the user into the Reddit API"""
    reddit = OAuth2Session(
        filters.CLIENT_ID, scope=['history', 'save'],
        redirect_uri=request.build_absolute_uri(
            reverse('compilations-callback')))
    authorization_url, state = reddit.authorization_url(AUTH_ENDPOINT)
    request.session['oauth_state'] = state
    return redirect(authorization_url)

@login_required
def callback(request):
    """OAuth2 Callback"""
    reddit = OAuth2Session(
        filters.CLIENT_ID, state=request.session['oauth_state'],
        redirect_uri=request.build_absolute_uri(
            reverse('compilations-callback')))
    request.session['oauth_token'] = reddit.fetch_token(
        TOKEN_ENDPOINT, client_secret=filters.CLIENT_SECRET,
        client_id=filters.CLIENT_ID,
        authorization_response=request.build_absolute_uri(),
        headers={'User-Agent': USER_AGENT})
    return redirect('compilations-index')

class VideoCollectionView(viewsets.ViewSet):
    authentication_classes = [SessionAuthentication]
    permission_classes = [IsAuthenticated]
    lookup_field = 'video'

    def list(self, request):
        """Return a list of videos"""
        if 'oauth_token' not in request.session:
            return Response(
                status=400,
                data={'message': 'no valid OAuth2 token in session'}
            ) # Bad request
        reddit = OAuth2Session(
            filters.CLIENT_ID, token=request.session['oauth_token'])
        if 'count' in request.GET and 'after' in request.GET:
            count = int(request.GET.get('count'))
            after = request.GET.get('after')
            logging.error('Query params: count=%d, after=%s', count, after)
            response = reddit.get(
                REDDIT_BASE + f'/user/{filters.REDDIT_USER}/saved'
                + f'?count={count}&after={after}',
                headers={'User-Agent': USER_AGENT}
            )
        else:
            count = 0
            logging.error('No query params')
            response = reddit.get(
                REDDIT_BASE + f'/user/{filters.REDDIT_USER}/saved',
                headers={'User-Agent': USER_AGENT}
            )
        if not response.ok:
            return Response(
                status=500,
                data={'message': 'Error ' + response.status_code
                      + ' while retrieving saved videos: '}
            )
        videos = json.loads(response.text)
        if not videos['data']['children']:
            return Response(status=404) # No more!
        return Response(
            {
                'count': count + int(videos['data']['dist']),
                'after': videos['data']['after'],
                'videos': [
                    {
                        'url': x['data']['url'],
                        'name': x['data']['name'],
                        'guid': b64encode(x['data']['url'].encode('utf-8'))
                    } for x in
                    filter(lambda y: 'domain' in y['data'] \
                           and any(y['data']['domain'] == z['domain'] and \
                                   z['filter'](y['data'])
                                   for z in filters.DOMAINS),
                           videos['data']['children'])
                ],
            }
        )

    def retrieve(self, request, video):
        """Return the URL for a video"""
        # NOTE: In this case, video is the base64 encoding of the actual video
        #       URL. This is different from the .delete() endpoint, because DRF
        #       does not have great support for taking different kinds of keys.
        link = b64decode(video).decode('utf-8')
        response = requests.get(link)
        if not response.ok:
            return Response(
                status=response.status_code,
                content_type=response.headers['Content-Type'],
                headers=response.headers,
                data=response.text,
            )
        for domain in filters.DOMAINS:
            if domain['domain'] in link:
                return Response(domain['handler'](response))
        return Response(status=404) # If we got here, something happened.

    def delete(self, request, video):
        """Unsave a video"""
        # NOTE: In this case, video is the "fullname" of the post.
        if 'oauth_token' not in request.session:
            return Response(
                status=400,
                data={'message': 'no valid OAuth2 token in session'}
            ) # Bad request
        reddit = OAuth2Session(
            filters.CLIENT_ID, token=request.session['oauth_token'])
        response = reddit.post(REDDIT_BASE + f'/api/unsave?id={video}',
                           headers={
                               'User-Agent': USER_AGENT,
                               # 'X-Modhash': request.META['X-Modhash'],
                           },
        )
        if not response.ok:
            logging.error('compilations: %d', response.status_code)
            logging.error('compilations: %s', response.text)
        return Response(
            status=response.status_code,
            content_type=response.headers['Content-Type'],
            headers=response.headers,
            data=response.text
        )
