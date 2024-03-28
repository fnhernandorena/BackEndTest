from pymongo import MongoClient
import os

coll = MongoClient(os.getenv('MONGO_URI'))['test']['shoes_python_flask']