use rocket::{response::content::RawHtml, serde::json::{json, Json, Value}};
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
async fn find_by_id(db: DbConn, id: i32) -> Value {
    db.run(move |c| {
        // let link = links::table::find(links::table, id)
        //     .get_result::<Link>(c)
        //     .expect("Error while findind link in the database.");

        if let Ok(link) = links::table::find(links::table, id).get_result::<Link>(c) {
            json!(link)
        } else {
            json!("Not Found")
        }
    })
    .await
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
async fn delete_link(db: DbConn, id: i32) -> Value {
    db.run(move |c| {
        // let result = diesel::delete(links::table.find(id))
        //     .execute(c)
        //     .expect("Error deleting from the database.");

        if let Ok(result) = diesel::delete(links::table.find(id)).execute(c) {
            match result {
               1 => json!("User deleted"),
               _ => json!("User not found") 
            }
        }else{
            json!("Error deleting user from the database")
        }

        // json!(result)
    })
    .await
}

// #[get("/")]
// fn index() -> RawHtml<&'static str> {
//     RawHtml(r#"<h1>Hello world</h1>"#)
// }

//CATCHERS
#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![all, find_by_id, save_link, delete_link])
        .register("/", catchers![not_found])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
