use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ClientOptions;
use mongodb::{self, Client, Collection};

use crate::commands::structures::{Add, Cmd, Command};

use super::errors::ModelError;

pub async fn connect_db(ip: &str) -> Result<Client, ModelError> {
    let client_options = ClientOptions::parse(ip)
        .await
        .map_err(|_| ModelError::InternalServerError)?;

    Ok(Client::with_options(client_options).unwrap())
}

// async fn create_collection(client: &Client, db_name: &str, collection_name: &str) {
//     let db = client.database(db_name);
//     db.create_collection(collection_name).await.unwrap();
// }

pub async fn insert_data(
    client: &Client,
    db_name: &str,
    collection_name: Command,
    data: Add,
) -> ObjectId {
    let db = client.database(db_name);
    let coll = db.collection(&collection_name.to_string());
    let doc = doc! {
    "description": data.description,
    "amount": data.amount,
    };
    let obj_id = coll.insert_one(doc).await.unwrap();
    obj_id.inserted_id.as_object_id().unwrap().to_owned()
}

pub async fn get_all_data(client: &Client, db_name: &str, collection_name: &str) -> Vec<Cmd> {
    let db = client.database(db_name);
    let coll: Collection<Cmd> = db.collection(collection_name);
    let mut cursor = coll.find(doc! {}).await.unwrap();
    let mut data: Vec<Cmd> = Vec::new();
    while cursor.advance().await.unwrap() {
        data.push(cursor.deserialize_current().unwrap())
    }

    data
}

pub async fn get_data(client: &Client, db_name: &str, collection_name: &str, id: i32) -> Cmd {
    let db = client.database(db_name);
    let coll: Collection<Cmd> = db.collection(collection_name);
    let doc = doc! {
    "id": id,
    };
    coll.find_one(doc).await.unwrap().unwrap()
}

pub async fn delete_data(client: &Client, db_name: &str, collection_name: &str, id: i32) {
    let db = client.database(db_name);
    let coll: Collection<Cmd> = db.collection(collection_name);
    let doc = doc! {
    "id": id,
    };
    coll.delete_one(doc).await.unwrap();
}

pub async fn update_data(
    client: &Client,
    db_name: &str,
    collection_name: &str,
    id: i32,
    data: Add,
) {
    let db = client.database(db_name);
    let coll: Collection<Cmd> = db.collection(collection_name);
    let filter = doc! {
    "id": id,
    };
    let update = doc! {
    "$set": {
    "description": data.description,
    "amount": data.amount,
    },
    };
    coll.update_one(filter, update).await.unwrap();
}
