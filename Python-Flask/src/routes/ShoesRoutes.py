from flask import Blueprint

from controllers.ShoesController import get_shoes_controller, create_shoe_controller, get_shoe_controller, delete_shoe_controller, update_shoe_controller

shoes = Blueprint('shoes', __name__)

@shoes.route('/', methods=['GET'])
def get_shoes():
  return get_shoes_controller()

@shoes.route('/<id>', methods=['GET'])
def get_shoe(id):
  return get_shoe_controller(id)

@shoes.route('/', methods=['POST'])
def create_shoe():
  return create_shoe_controller()

@shoes.route('/<id>', methods=['PUT'])
def update_shoe(id):
  return update_shoe_controller(id)

@shoes.route('/<id>', methods=['DELETE'])
def delete_shoe(id):
  return delete_shoe_controller(id)