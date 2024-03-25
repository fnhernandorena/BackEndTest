mod controllers;
mod models;
mod repositories;

#[macro_use]
extern crate rocket;
use controllers::shoe_controller::{create_shoe, get_shoe, update_shoe, delete_shoe, get_all_shoes};
use repositories::mongodb_repo::MongoRepo;


#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db)
    .mount("/", routes![get_all_shoes])
    .mount("/", routes![create_shoe])
    .mount("/", routes![get_shoe])
    .mount("/", routes![update_shoe])
    .mount("/", routes![delete_shoe])
}