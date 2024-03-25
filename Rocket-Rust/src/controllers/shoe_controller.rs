use crate::{models::shoe_model::Shoe , repositories::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult, bson::oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};

#[get("/shoes")]
pub fn get_all_shoes(db: &State<MongoRepo>) -> Result<Json<Vec<Shoe>>, Status> {
    let shoes = db.get_all_shoes();
    match shoes {
        Ok(shoe_list) => Ok(Json(shoe_list)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/shoes", data="<new_shoe>")]
pub fn create_shoe(
    db: &State<MongoRepo>,
    new_shoe: Json<Shoe>,) -> Result<Json<InsertOneResult>, Status>{
    let data = Shoe {
        id: None,
        brand: new_shoe.brand.to_owned(),
        model: new_shoe.model.to_owned(),
        size: new_shoe.size.to_owned()
    };
    let shoe_detail = db.create_shoe(data);
    match shoe_detail {
        Ok(shoe) => Ok(Json(shoe)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/shoes/<path>")]
pub fn get_shoe(db: &State<MongoRepo>, path: String) -> Result<Json<Shoe>,Status>{
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest)
    };
    let shoe_detail = db.get_shoe(&id);
    match shoe_detail {
        Ok(shoe) => Ok(Json(shoe)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[put("/shoes/<path>", data = "<new_shoe>")]
pub fn update_shoe(
    db: &State<MongoRepo>,
    path: String,
    new_shoe:Json<Shoe>,)-> Result<Json<Shoe>,Status>{
    let id = path;
    if id.is_empty(){
        return Err(Status::BadRequest)
    };
    let data = Shoe {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        brand: new_shoe.brand.to_owned(),
        model: new_shoe.model.to_owned(),
        size: new_shoe.size.to_owned()
    };

    let update_result = db.update_shoe(&id, data);
    match update_result{
        Ok(update) => {
            if update.matched_count ==1 {
                let updated_shoe_info = db.get_shoe(&id);
                return match updated_shoe_info {
                    Ok(shoe) => Ok(Json(shoe)),
                    Err(_) => Err(Status::InternalServerError)
                };
            } else {
                return Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/shoes/<path>")]
pub fn delete_shoe(
    db: &State<MongoRepo>,
    path: String) -> Result<Json<&str>, Status>{
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest)
        };
        let result = db.delete_shoe(&id);
        match result {
            Ok(res) => {
                if res.deleted_count == 1 {
                    return Ok(Json("Shoe deleted Successfully!"));
                } else {
                    return Err(Status::NotFound);
                }
            }
            Err(_) => Err(Status::InternalServerError)
        }

}