from django.shortcuts import render
from shoes_repo import coll
from django.http import JsonResponse, Http404
from bson import json_util, ObjectId
import json

# Create your views here.


def get_all_shoes(request):
    shoes = coll.find()
    shoes_list = [shoe for shoe in shoes]
    for shoe in shoes_list:
        shoe['_id'] = str(shoe['_id'])
    return JsonResponse(shoes_list, safe=False, json_dumps_params={'default': json_util.default})

def get_shoe_by_id(request, shoe_id):
    shoe = coll.find_one({'_id': ObjectId(shoe_id)})
    if shoe is None:
        raise Http404("Shoe does not exist")
    shoe['_id'] = str(shoe['_id'])
    return JsonResponse(shoe, safe=False, json_dumps_params={'default': json_util.default})

def create_shoe(request):
    if request.method == 'POST':
        data = json.loads(request.body)
        # Crear un nuevo documento en la colección
        shoe_data = {
            'brand': data['brand'],
            'model': data['model'],
            'size': data['size']
        }
        # Insertar el nuevo documento en la colección y obtener su ID
        result = coll.insert_one(shoe_data)
        # Obtener el ID del documento insertado
        shoe_id = str(result.inserted_id)
        return JsonResponse({'message': 'Shoe created successfully', 'shoe_id': shoe_id}, status=201)
    
    else:
        return JsonResponse({'error': 'Method not allowed'}, status=405)
    
def update_shoe(request, shoe_id):
    if request.method == 'PUT':  # Método PUT para la actualización
        data = json.loads(request.body)
        # Verificar si el zapato existe en la base de datos
        if coll.find_one({'_id': ObjectId(shoe_id)}) is None:
            raise Http404("Shoe does not exist")
        # Actualizar el documento en la colección
        coll.update_one({'_id': ObjectId(shoe_id)}, {'$set': data})
        return JsonResponse({'message': 'Shoe updated successfully'}, status=200)
    else:
        return JsonResponse({'error': 'Method not allowed'}, status=405)

def delete_shoe(request, shoe_id):
    if request.method == 'DELETE':  # Método DELETE para la eliminación
        # Verificar si el zapato existe en la base de datos
        if coll.find_one({'_id': ObjectId(shoe_id)}) is None:
            raise Http404("Shoe does not exist")
        # Eliminar el documento de la colección
        coll.delete_one({'_id': ObjectId(shoe_id)})
        return JsonResponse({'message': 'Shoe deleted successfully'}, status=200)
    else:
        return JsonResponse({'error': 'Method not allowed'}, status=405)