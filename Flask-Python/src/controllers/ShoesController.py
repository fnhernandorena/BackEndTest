from flask import request, Response
from bson import json_util, ObjectId
from pymongo import MongoClient

from repositories.ShoesRepository import coll
 
def create_shoe_controller():
    data = request.get_json()
    brand = data.get("brand", None)
    model = data.get("model", None)
    size = data.get("size", None)
    if brand:
      response = coll.insert_one({
        "brand": brand,
        "model": model,
        "size": size,
      })
      result = {
        "id": str(response.inserted_id),
        "brand": brand,
        "model": model,
        "size": size
      }
      return result
    else:
      return 'Invalid payload', 400
    
def get_shoes_controller():
    data = coll.find()
    result = json_util.dumps(data)
    return Response(result, mimetype='application/json')

def get_shoe_controller(id):
    data = coll.find_one({'_id': ObjectId(id)})
    result = json_util.dumps(data)
    return Response(result, mimetype='application/json')

def update_shoe_controller(id):
    data = request.get_json()
    if len(data) == 0:
      return 'Invalid payload', 400
    
    response = coll.update_one({'_id': ObjectId(id)}, {'$set': data})

    if response.modified_count >= 1:
        return 'Shoe updated successfully', 200
    else:
        return 'Shoe not found', 404

def delete_shoe_controller(id):
    response = coll.delete_one({'_id': ObjectId(id)})
    if response.deleted_count >= 1:
        return 'Shoe deleted successfully', 200
    else:
        return 'Shoe not found', 404