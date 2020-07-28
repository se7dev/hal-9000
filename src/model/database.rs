extern crate mongodb;
<<<<<<< HEAD
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
=======

use mongodb::{options::ClientOptions, Client};

async fn db_connection() -> Result<mongodb::Database, mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment.
    for db_name in client.list_database_names(None, None).await {
        println!("{:#?}", db_name);
    }

    // Get a handle to a database.
    let db = client.database("mydb");
    println!("executed");
    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await {
        println!("{:#?}", collection_name);
    }

    match db {
        db => Ok(db),
    }
}

fn main() {
    println!("aksjnaksdn");
    let db_result = db_connection();
    println!("ASDSFASF");

}
>>>>>>> 730c9ee361b469451ddab53578c3c8b34ac4d070
