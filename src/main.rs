#[macro_use]
extern crate rocket;

use dynomite::dynamodb::*;
use rusoto_core::Region;

pub mod errors;
use errors::*;

mod products;

#[launch]
fn rocket() -> rocket::Rocket {
    let db = DynamoDbClient::new(Region::CnNorth1);
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/products", products::get_routes())
        .manage(db)
}

#[get("/")]
async fn index() -> Result<String> {
    Ok("Hello World".to_string())
}
