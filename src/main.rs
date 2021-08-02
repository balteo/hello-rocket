#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use rocket::{get, routes};
use rocket_contrib::json::Json;
use rocket::response::status::NotFound;

use hello_rocket::establish_connection;
use hello_rocket::models::Person;
use hello_rocket::schema::person::dsl::person;

#[get("/<id>", format = "json")]
fn find_one(id: i32) -> Result<Json<Person>, NotFound<String>> {
    let connection = establish_connection();

    let one: QueryResult<Person> = person.find(id).first(&connection);

    println!("{:?}", one);

    match one {
        Ok(one) => Ok(Json(one)),
        Err(one)=> Err(NotFound(one.to_string()))
    }
}

#[get("/", format = "json")]
fn find_all() -> Json<Vec<Person>> {
    let connection = establish_connection();

    let all: QueryResult<Vec<Person>> = person::table().get_results(&connection);

    println!("{:?}", all);

    Json(all.ok().unwrap())
}

fn main() {
    rocket::ignite().mount("/", routes![find_one, find_all]).launch();
}
