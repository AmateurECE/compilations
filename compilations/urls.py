from django.urls import path
from rest_framework import routers
from rest_framework.renderers import JSONRenderer
from rest_framework.authentication import SessionAuthentication
from rest_framework.permissions import IsAuthenticated

from .views import VideoCollectionView, index, login, callback

urlpatterns = [
    path('', index, name='compilations-index'),
    path('login/', login),
    path('callback/', callback, name='compilations-callback'),
]

router = routers.SimpleRouter()
router.register(r'videos', VideoCollectionView, basename='videos')
urlpatterns += router.urls
