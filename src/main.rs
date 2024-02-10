use diesel::result::Error::NotFound;
use rocket::{
    response::status::Custom,
    serde::json::{json, Json, Value},
};
mod controllers;
mod models;
mod schema;
use crate::models::NewLink;
use controllers::Controller;
use rocket::http::Status;
use rocket_sync_db_pools::database;

#[macro_use]
extern crate rocket;

#[database("sqlite")]

struct DbConn(diesel::SqliteConnection);

#[get("/")]
async fn all(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| match Controller::find_many(c) {
        Ok(links) => Ok(json!(links)),
        Err(e) => Err(Custom(Status::InternalServerError, json!(e.to_string()))),
    })
    .await
}

#[get("/<id>")]
async fn find_by_id(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| match Controller::find_one(c, id) {
        Ok(link) => Ok(json!(link)),
        Err(e) => {
            println!("{:?}", e);
            match e {
                NotFound => Err(Custom(Status::NotFound, json!(e.to_string()))),
                _ => Err(Custom(Status::InternalServerError, json!(e.to_string()))),
            }
        }
    })
    .await
}

#[post("/", format = "json", data = "<new_link>")]
async fn save_link(new_link: Json<NewLink>, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| match Controller::create(c, new_link.into_inner()) {
        Ok(link) => Ok(json!(link)),
        Err(e) => Err(Custom(Status::InternalServerError, json!(e.to_string()))),
    })
    .await
}

#[delete("/<id>")]
async fn delete_link(db: DbConn, id: i32) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        match Controller::delete(c, id) {
            Ok(_) => Ok(json!("User deleted")),
            Err(e) => {
                match e {
                   NotFound =>  Err(Custom(Status::NotFound, json!(e.to_string()))), 
                   _ => Err(Custom(Status::InternalServerError, json!(e.to_string()))), 
                }
            }
        }
    })
    .await
}

//CATCHERS
#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}
#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized")
}
#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("Unprocessable entity")
}
#[catch(400)]
fn bad_request() -> Value {
    json!("Bad request")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![all, find_by_id, save_link, delete_link])
        .register(
            "/",
            catchers![not_found, unprocessable_entity, unauthorized, bad_request],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}
