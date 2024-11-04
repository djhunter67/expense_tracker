use mongodb::bson::doc;
use mongodb::options::ClientOptions;
use mongodb::{self, Client, Collection};

use crate::commands::structures::Add;

async fn connect_db(ip: &str) -> Client {
    let client_options = ClientOptions::parse(ip).await.unwrap();
    Client::with_options(client_options).unwrap()
}

async fn create_collection(client: &Client, db_name: &str, collection_name: &str) {
    let db = client.database(db_name);
    db.create_collection(collection_name).await.unwrap();
}

async fn insert_data(client: &Client, db_name: &str, collection_name: &str, data: Add) {
    let db = client.database(db_name);
    let collection: Collection<Add> = db.collection::<Add>(&data.description.to_string());
    let doc = doc! {
    "description": data.description,
    "amount": data.amount,
    };
    collection
        .insert_one(doc! {
            "description": data.description,
            "amount": data.amount,
        })
        .await
        .unwrap();
}
