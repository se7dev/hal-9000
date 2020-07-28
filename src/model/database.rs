extern crate mongodb;
extern crate tokio;

use chrono::{Datelike, Utc};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    error::Result,
    Client,
};

struct Message {
    message: String,
    user: String,
    timestamp: String,
}

struct Logs {
    messages: Vec<Message>,
}

fn build_date() -> String {
    let now = Utc::now();
    let mut time: String = "".to_owned();
    time.push_str(&now.day().to_string());
    time.push_str("-");
    time.push_str(&now.month().to_string());
    time.push_str("-");
    time.push_str(&now.year_ce().1.to_string());
    return time;
}

#[tokio::main]
async fn get_logs(date: String) -> Result<()> {
    let mut logs = Logs {
        messages: Vec::new(),
    };

    // Create the client by passing in a MongoDB connection string.
    //
    // When connecting to Atlas, the connection string prefixed with
    // "mongodb+srv" can be used in this manner.
    let client = Client::with_uri_str("mongodb://root:root@mongodb:27017/").await?;
    // Get a handle to the collection being used.
    let db = client.database("logs");
    let time = build_date();

    let coll = db.collection(&time);

    // Query the database for all messages which are on that date.
    let mut cursor = coll.find(doc! { "date:": time }, None).await?;

    // Iterate over each document in the cursor, using serde to
    // deserialize them into Pets.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(message) = document.get("message").and_then(Bson::as_str) {
                    println!("message: {}", message);
                } else {
                    println!("no messages found");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

async fn write_logs(logs: Logs) -> Result<String> {
    // Create the client by passing in a MongoDB connection string.
    //
    // When connecting to Atlas, the connection string prefixed with
    // "mongodb+srv" can be used in this manner.
    let client = Client::with_uri_str("mongodb://root:root@mongodb:27017/").await?;
    // Get a handle to the collection being used.
    let db = client.database("logs");
    let time = build_date();

    let coll = db.collection(&time);
    for message in logs.messages {
        coll.insert_one(doc! { "message": message.message, "user": message.user, "timestamp" : message.timestamp },None).await?;
    }
    // Insert the document into the database.
    return Ok("Ok".to_owned());
}

fn main() {}
