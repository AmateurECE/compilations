from django.urls import path
from rest_framework import routers
from rest_framework.schemas import get_schema_view
from rest_framework.renderers import JSONRenderer

from .views import VideoCollectionView, index

urlpatterns = [
    path('', index),
]

router = routers.SimpleRouter()
router.register(r'videos', VideoCollectionView, basename='videos')
urlpatterns += router.urls

urlpatterns += [
    path('api/', get_schema_view(
        title='Compilations',
        description='API for Compilations',
        version='1.0.0',
        renderer_classes=[JSONRenderer]
    ), name='api-schema'),
]
