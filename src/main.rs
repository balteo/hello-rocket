#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use rocket::{get, routes};
use rocket_contrib::json::Json;

use hello_rocket::establish_connection;
use hello_rocket::models::Person;
use hello_rocket::schema::person::dsl::person;

#[get("/", format = "json")]
fn index() -> Json<Person> {
    let connection = establish_connection();

    let me: QueryResult<Person> = person::table().first::<Person>(&connection);

    println!("{:?}", me);

    Json(me.ok().unwrap())
}

#[get("/all", format = "json")]
fn all() -> Json<Vec<Person>> {
    let connection = establish_connection();

    let all: QueryResult<Vec<Person>> = person::table().get_results(&connection);

    println!("{:?}", all);

    Json(all.ok().unwrap())
}

fn main() {
    rocket::ignite().mount("/", routes![index, all]).launch();
}
