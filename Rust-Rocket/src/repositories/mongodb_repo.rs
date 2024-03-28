use mongodb::{
    bson::{ extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection}
};

use dotenv::dotenv;

use std::env;

use crate::models::shoe_model::Shoe;

pub struct MongoRepo {
    col: Collection<Shoe>
}

impl MongoRepo{
    pub fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGO_URI")
        .expect("La variable de entorno MONGODB_URI no está definida");
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("test");
        let col: Collection<Shoe> = db.collection("shoes_rust_rocket");
        MongoRepo { col }
   }

   pub fn get_all_shoes(&self) -> Result<Vec<Shoe>, mongodb::error::Error> {
    let cursor = self.col.find(None,None)?;

    // Iteramos sobre el cursor para obtener todos los documentos
    let mut shoes = Vec::new();
    for result in cursor {
        let shoe_doc = result?;
        shoes.push(shoe_doc);
    }

    Ok(shoes)
}

   pub fn create_shoe(&self, new_shoe: Shoe) -> Result<InsertOneResult, Error>{
        let new_doc = Shoe {
            id: None,
            brand: new_shoe.brand,
            model: new_shoe.model,
            size: new_shoe.size

        };
        let shoe = self
        .col
        .insert_one(new_doc, None)
        .ok()
        .expect("Error Creating shoe");
        Ok(shoe)
   }

   pub fn get_shoe(&self, id: &str) -> Result<Shoe, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id" : object_id};
        let shoe_detail = self.col
        .find_one(filter,None).ok().expect("Error getting shoe info");
        Ok(shoe_detail.unwrap())
   }

   pub fn update_shoe(&self, id:&str, updated_shoe: Shoe) -> Result<UpdateResult, Error>{
    let object_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id" : object_id};
    let new_doc = doc !{
        "$set": {
            "brand" : updated_shoe.brand,
            "model" : updated_shoe.model,
            "size" : updated_shoe.size
        }
    };
    let update_doc = self.col
    .update_one(filter, new_doc, None)
    .ok()
    .expect("Error updating shoe");
    Ok(update_doc)
   }

   pub fn delete_shoe(&self, id:&str) -> Result<DeleteResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let shoe_detail = self.col
        .delete_one(filter, None)
        .ok()
        .expect("Error deleting shoe");
    Ok(shoe_detail)
   }
}