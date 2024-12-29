use std::fmt::{self, Display, Formatter};

use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ClientOptions;
use mongodb::sync::{Client, Collection};

use crate::commands::structures::{Add, Cmd};

use super::errors::ModelError;

pub fn connect_db(ip: &str) -> Result<Client, ModelError> {
    let client_options = ClientOptions::parse(ip).run();

    Ok(Client::with_options(client_options?).unwrap())
}

// async fn create_collection(client: &Client, db_name: &str, collection_name: Collected) {
//     let db = client.database(db_name);
//     db.create_collection(collection_name).await.unwrap();
// }

pub async fn insert_data(
    client: &Client,
    db_name: &str,
    collection_name: Collected,
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

pub async fn get_all_data(client: &Client, db_name: &str, collection_name: Collected) -> Vec<Cmd> {
    let db = client.database(db_name);
    let coll: Collection<Cmd> = db.collection(&collection_name.to_string());
    let mut cursor = coll.find(doc! {}).await.unwrap();
    let mut data: Vec<Cmd> = Vec::new();
    while cursor.advance().await.unwrap() {
        data.push(cursor.deserialize_current().unwrap())
    }

    data
}

pub async fn get_data(client: &Client, db_name: &str, collection_name: Collected, id: i32) -> Cmd {
    let db = client.database(db_name);
    let coll: Collection<Cmd> = db.collection(&collection_name.to_string());
    let doc = doc! {
    "id": id,
    };
    coll.find_one(doc).await.unwrap().unwrap()
}

pub async fn delete_data(client: &Client, db_name: &str, collection_name: Collected, id: i32) {
    let db = client.database(db_name);
    let coll: Collection<Cmd> = db.collection(&collection_name.to_string());
    let doc = doc! {
    "id": id,
    };
    coll.delete_one(doc).await.unwrap();
}

pub async fn update_data(
    client: &Client,
    db_name: &str,
    collection_name: Collected,
    id: i32,
    data: Add,
) {
    let db = client.database(db_name);
    let coll: Collection<Cmd> = db.collection(&collection_name.to_string());
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

#[derive(Debug)]
pub enum Collected {
    Add,
    List,
    Summary,
    Delete,
}

impl Display for Collected {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Collected::Add => write!(f, "add"),
            Collected::List => write!(f, "list"),
            Collected::Summary => write!(f, "summary"),
            Collected::Delete => write!(f, "delete"),
        }
    }
}
