use rocket::serde::json::{json, Json, Value};
mod models;
mod schema;
use crate::{
    models::{Link, NewLink},
    schema::links,
};
use diesel::prelude::*;
use rocket_sync_db_pools::database;

#[macro_use]
extern crate rocket;

#[database("sqlite")]

struct DbConn(diesel::SqliteConnection);

#[get("/")]
async fn all(db: DbConn) -> Value {
    db.run(|c| {
        let links = links::table
            .order(links::id.desc())
            .limit(1000)
            .load::<Link>(c)
            .expect("Error while retriving all the links from the database.");

        json!(links)
    })
    .await
}

#[get("/<id>")]
fn find_by_id(id: i32) -> Value {
    json!(id)
}

#[post("/", format = "json", data = "<new_link>")]
async fn save_link(new_link: Json<NewLink>, db: DbConn) -> Value {
    db.run(|c| {
        let link = diesel::insert_into(links::table)
            .values(new_link.into_inner())
            .execute(c)
            .expect("Error while inserting a new created link in the database.");
        json!(link)
    })
    .await
}

#[delete("/<id>")]
fn delete_link(id: i32) -> Value {
    json!(id)
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![all, find_by_id, save_link, delete_link])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
