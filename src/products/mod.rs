use dynomite::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod routes;

pub fn table_name() -> String {
    String::from("products")
}

#[derive(Debug, Serialize, Deserialize, Item, Clone)]
pub struct Product {
    #[dynomite(partition_key)]
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
}

impl Default for Product {
    fn default() -> Self {
        Product {
            id: Some(Uuid::new_v4()),
            name: "".to_string(),
            description: "".to_string(),
        }
    }
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![routes::list, routes::create, routes::delete, routes::get]
}
