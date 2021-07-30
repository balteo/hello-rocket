use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
}
