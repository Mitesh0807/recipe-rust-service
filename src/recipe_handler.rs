// use axum::{extract, http};
// use mongodb::bson::doc;
// use mongodb::bson::Document;
// use mongodb::{Client, Collection, Database, InsertOneResult};
// // use serde::{Deserialize, Serialize};
// pub async fn health_check(extract::State(db): extract::State<Database>) -> http::StatusCode {
//     let collection: Collection<serde_json::Value> = db.collection("categories");
//     // let count = collection.find(None, None).await;
//     // let count = count.unwrap();
//     // println!("{:?}", count);
//     let new_doc: Document = doc! {
//         "name": "Test",
//         "description": "Test",
//         "img_Base64": "Test",
//         "subName": "Test",
//         "createdAt": "2022-10-01T00:00:00.000Z",
//         "updatedAt": "2022-10-01T00:00:00.000Z",
//         "isActive": true,
//         "slug": "test"
//     };
//     let res: Result<InsertOneResult, mongodb::error::Error> =
//         collection.insert_one(new_doc.clone(), None).await;
//     println!("{:?}", res.inserted_id);
//     http::StatusCode::OK
// }

use axum::{extract, http};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson};
use mongodb::{Client, Collection, Database};
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

pub async fn get_categories(
    extract::State(db): extract::State<Database>,
    extract::Path(id): extract::Path<String>,
) -> http::StatusCode {
    let collection_id = ObjectId::parse_str(&id).unwrap();
    let collection: Collection<Value> = db.collection("categories");
    let get_result = collection.find_one(doc! {"_id": collection_id}, None).await;
    match get_result {
        Ok(result) => http::StatusCode::OK,
        Err(e) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}
