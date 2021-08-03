#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use rocket::{get, routes};
use rocket::response::status::NotFound;
use rocket_contrib::databases::diesel;
use rocket_contrib::json::Json;

use hello_rocket::models::Person;
use hello_rocket::schema::person::dsl::person;

#[get("/<id>", format = "json")]
fn find_one(conn: PgDbConn, id: i32) -> Result<Json<Person>, NotFound<String>> {
    let one: QueryResult<Person> = person.find(id).first(&*conn);

    println!("{:?}", one);

    match one {
        Ok(one) => Ok(Json(one)),
        Err(one) => Err(NotFound(one.to_string()))
    }
}

#[get("/", format = "json")]
fn find_all(conn: PgDbConn) -> Json<Vec<Person>> {
    let all: QueryResult<Vec<Person>> = person::table().get_results(&*conn);

    println!("{:?}", all);

    Json(all.ok().unwrap())
}

#[database("pg_db")]
struct PgDbConn(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .attach(PgDbConn::fairing())
        .mount("/", routes![find_one, find_all])
        .launch();
}
