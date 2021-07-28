from django.shortcuts import render
from rest_framework import viewsets
from rest_framework.response import Response
import requests
from bs4 import BeautifulSoup

import json
import logging

VIDEOS = [
]

def index(request):
    return render(request, 'compilations/index.html')

class VideoCollectionView(viewsets.ViewSet):
    def list(self, request):
        """Return a list of videos"""
        return Response(VIDEOS)

    def retrieve(self, request, pk=None):
        """Return the URL for a video"""
        try:
            key = int(pk)
            if key >= len(VIDEOS) or key < 0:
                return Response(status=404) # Not found
            response = requests.get(VIDEOS[key]['url'])
            if response.ok:
                soup = BeautifulSoup(response.text, 'html.parser')
                videoElement = soup.find('meta', {'property': 'og:video'})
                return Response(videoElement['content'])
        except ValueError:
            return Response(status=400) # Bad request
