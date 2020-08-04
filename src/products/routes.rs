use dynomite::dynamodb::*;
use dynomite::*;
use rocket::State;
use rocket_contrib::json::Json;
use uuid::Uuid;

use super::Product;
use crate::errors::*;

// CREATE: This handler is responsible for creating new product items
// POST /products
#[post("/", format = "json", data = "<product>")]
pub async fn create(
    db: State<'_, DynamoDbClient>,
    product: Json<Product>,
) -> Result<Json<Product>> {
    let p = Product {
        id: Some(Uuid::new_v4()),
        ..product.0
    };

    db.put_item(PutItemInput {
        table_name: super::table_name(),
        item: p.clone().into(),
        ..PutItemInput::default()
    })
    .await
    .map(|_| Json(p))
    .map_err(|e| Error::InternalError(e.to_string()))
}

// LIST: This handler is responsible for listing all products
// GET /products
#[get("/")]
pub async fn list(db: State<'_, DynamoDbClient>) -> Result<Json<Vec<Product>>> {
    let products: Vec<Product> = db
        .scan(ScanInput {
            table_name: String::from(super::table_name()),
            ..ScanInput::default()
        })
        .await
        .map_err(|e| Error::InternalError(e.to_string()))
        .map(|output| {
            output
                .items
                .unwrap_or_default()
                .into_iter()
                .flat_map(|item| Product::from_attrs(item.clone()))
                .collect()
        })?;

    Ok(Json(products))
}

// DELETE: This handler is responsible for deleting a single product
// DELETE /products/<id>
#[delete("/<id>")]
pub async fn delete(db: State<'_, DynamoDbClient>, id: String) -> Result<()> {
    let id: Uuid =
        Uuid::parse_str(&id).map_err(|_| Error::BadRequestError("invalid id".to_string()))?;

    let product = Product {
        id: Some(id),
        ..Product::default()
    };

    db.delete_item(DeleteItemInput {
        table_name: super::table_name(),
        key: product.key(),
        ..DeleteItemInput::default()
    })
    .await
    .map_err(|e| Error::InternalError(e.to_string()))?;

    Ok(())
}
// GET: This handler is responsible for getting a single product
// GET /products/<id>
#[get("/<id>")]
pub async fn get(db: State<'_, DynamoDbClient>, id: String) -> Result<Json<Product>> {
    let id: Uuid =
        Uuid::parse_str(&id).map_err(|_| Error::BadRequestError("invalid id".to_string()))?;

    let product = Product {
        id: Some(id),
        ..Product::default()
    };

    let result = db
        .get_item(GetItemInput {
            table_name: super::table_name(),
            key: product.key(),
            ..GetItemInput::default()
        })
        .await
        .map_err(|e| Error::InternalError(e.to_string()))?;

    match result.item {
        Some(item) => {
            let product =
                Product::from_attrs(item).map_err(|e| Error::InternalError(e.to_string()))?;
            Ok(Json(product))
        }
        None => Err(Error::NotFoundError("not found".to_string())),
    }
}
