use axum::{extract, http};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson};
use mongodb::error::Error as MongoError;
use mongodb::{Client, Collection, Database};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub async fn health_check(extract::State(db): extract::State<Database>) -> http::StatusCode {
    let collection: Collection<Value> = db.collection("categories");
    let obj_id = "652cfd76001a4b612e589c21";
    let existing_doc_id = ObjectId::parse_str(obj_id).unwrap();
    let get_result = collection
        .find_one(doc! {"_id": existing_doc_id}, None)
        .await;
    println!("{:?}", get_result);

    match get_result {
        Ok(result) => {
            println!("{:?}", result);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    };
    http::StatusCode::OK
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    _id: ObjectId,
    name: String,
    description: String,
    img_Base64: String,
    subName: String,
    isActive: bool,
    slug: String,
}

pub async fn get_categories(
    extract::State(db): extract::State<Database>,
    extract::Path(id): extract::Path<String>,
) -> Result<axum::Json<Category>, http::StatusCode> {
    let collection_id = ObjectId::parse_str(&id).unwrap();
    let collection: Collection<Value> = db.collection("categories");
    let get_result = collection.find_one(doc! {"_id": collection_id}, None).await;
    match get_result {
        Ok(Some(result)) => match serde_json::from_value(result) {
            Ok(category) => Ok(axum::Json(category)),
            Err(e) => {
                eprintln!("Failed to deserialize category: {:?}", e);
                Err(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
        Ok(None) => Err(http::StatusCode::NOT_FOUND),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    name: String,
    description: String,
    img_base64: String,
    sub_name: String,
    is_active: bool,
    slug: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl CreateCategory {
    fn new(
        name: String,
        description: String,
        img_base64: String,
        sub_name: String,
        is_active: bool,
        slug: String,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            name,
            description,
            img_base64,
            sub_name,
            is_active: false,
            slug,
            created_at: now,
            updated_at: now,
        }
    }
}

pub async fn create_category(
    extract::State(db): extract::State<Database>,
    extract::Json(category): extract::Json<CreateCategory>,
) -> Result<http::StatusCode, http::StatusCode> {
    let collection: Collection<Value> = db.collection("categories");
    // let new = doc! {"name": category.name, "description": category.description};
    // println!("{:#?}", new)0
    let now = chrono::Utc::now();
    let new_doc = CreateCategory::new(
        category.name,
        category.description,
        category.img_base64,
        category.sub_name,
        category.is_active,
        category.slug,
        now,
        now,
    );
    println!("{:#?}", new_doc);
    // let new_doc = doc! {"$set": new_doc};
    // let insert_result = collection.insert_one(new_doc, None).await;
    // println!("{:#?}", insert_result);
    // match new_doc {
    //     Ok(_) => Ok(http::StatusCode::CREATED),
    //     Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    // }
    Ok(http::StatusCode::CREATED)
}
