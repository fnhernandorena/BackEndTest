from django.urls import path
from . import views

urlpatterns = [
    path('', views.get_all_shoes),
    path('id/<str:shoe_id>/', views.get_shoe_by_id),
    path('add/', views.create_shoe),
    path('delete/<str:shoe_id>/', views.delete_shoe),
    path('edit/<str:shoe_id>/', views.update_shoe),
]
