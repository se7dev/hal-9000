#![allow(warnings)]
extern crate mongodb;
extern crate tokio;


use chrono::{Datelike, Utc};
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Bson},
    error::Result,
    Client,
};
use std::borrow::Borrow;
// TODO: Make structs accessible from other packages

/// A Message must have a message, a user that sent the message and a timestamp (DD-MM-YY)
#[derive(Clone, Debug)]
struct Message {
    message: String,
    user: String,
    timestamp: String,
}

/// new_message simply constructs a new Message
impl Message {
    fn new_message(message: String, user: String, timestamp: String) -> Message {
        Message { message, user, timestamp }
    }
}

/// Logs hols multiple Messages in a Vector
#[derive(Clone, Debug)]
pub struct Logs {
    messages: Vec<Message>,
}

/// build_date() is a stringbuilder that gives back the current date as DD-MM-YY
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

/// get_logs() fetches all messages that have been sent on a specific date
pub async fn get_logs(date: String) -> Result<Logs> {
    let mut logs = Logs {
        messages: Vec::new(),
    };

    // Create the client by passing in a MongoDB connection string.
    let client = Client::with_uri_str("mongodb://root:root@database:27017/").await?;
    // Get a handle to the db and collection being used.
    let db = client.database("logs");
    let coll = db.collection(&date);

    // Query the database for all messages which are on that date.
    let mut cursor = coll.find(doc! {"timestamp": date.to_string()}, None).await?;
    // Iterate over each document in the cursor, using serde to
    // deserialize them into Messages.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let (Some(message), Some(user), Some(timestamp)) =
                (document.get("message").and_then(Bson::as_str),
                 document.get("user").and_then(Bson::as_str),
                 document.get("timestamp").and_then(Bson::as_str)) {
                    logs.messages.push(
                        Message::new_message(
                            message.to_string(),
                            user.to_string(),
                            timestamp.to_string()))
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    return Ok(logs);
}

/// write_logs() writes the collected messages into the database
pub async fn write_logs(logs: Logs) -> Result<String> {
    let client = Client::with_uri_str("mongodb://root:root@database:27017/").await?;
    let db = client.database("logs");
    let time = build_date();
    println!("{:?}", logs);

    let coll = db.collection(&logs.messages.get(0).unwrap().timestamp );
    for message in &logs.messages {
        let temp_message = message.clone();
        println!("Message: {:?}", temp_message);
        coll.insert_one(
            doc! {
                "message": temp_message.message,
                 "user": temp_message.user,
                  "timestamp" : temp_message.timestamp},
            None,
        ).await?;
    }

    // Insert the document into the database.
    return Ok("Ok".to_owned());
}

#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests {
    extern crate mongodb;
    extern crate tokio;

    use super::*;
    use chrono::{Datelike, Utc};
    use futures::stream::StreamExt;
    use mongodb::{
        bson::{doc, Bson},
        error::Result,
        Client,
    };
    use irc::client::prelude::Stream;
    use futures::TryFutureExt;
    use std::borrow::Borrow;

    const DATE: &str = "29-07-20";

    // Setup DB for tests
    async fn setup_db() -> Result<String> {
        let client = Client::with_uri_str("mongodb://root:root@database:27017/").await?;
        let db = client.database("logs");
        let coll = db.collection(DATE);
        let mut insertion = coll.insert_one(
            doc! {"message": "test message 1", "user": "test_user1", "timestamp" : "29-07-20" },
            None,
        )
            .await?;
        insertion = coll.insert_one(
            doc! {"message": "test message 2", "user": "test_user2", "timestamp" : "01-01-20" },
            None,
        )
            .await?;
        insertion = coll.insert_one(
            doc! {"message": "test message 3", "user": "test_user3", "timestamp" : "01-02-20" },
            None,
        )
            .await?;
        insertion = coll.insert_one(
            doc! {"message": "test message 4", "user": "test_user1", "timestamp" : "01-01-20" },
            None,
        )
            .await?;
        return Ok("Ok".to_owned());
    }

    async fn reset_db() -> Result<String> {
        let client = Client::with_uri_str("mongodb://root:root@database:27017/").await?;
        let db = client.database("logs");
        db.drop(None).await;
        return Ok("Ok".to_owned());
    }

    // // Helper Function to assert Result Type
    // fn check<T>(res: Result<T>) -> bool {
    //     match res {
    //         Ok(_) => true,
    //         Err(_) => false,
    //     }
    // }

    #[tokio::test]
    async fn reading_db_entries() {
        reset_db().await;
        setup_db().await;
        let logs = get_logs(DATE.to_string()).await;
        match logs {
            Ok(_) => {
                assert_ne!(logs.borrow().as_ref().unwrap().messages.len(), 0);
                for log in logs.unwrap().messages {
                    println!("Log found: \n Message {} \n from User {} \n at timestamp {}",
                             log.message, log.user, log.timestamp);
                    assert_eq!(log.timestamp, DATE);
                }
            }
            _ => println!("Message fetching did not work")
        }
    }

    #[tokio::test]
    async fn insert_db_entries() {
        reset_db().await;
        setup_db().await;
        let mut logs = Logs { messages: Vec::new() };
        logs.messages.push(
            Message::new_message(
                String::from("1"),
                String::from("InsertedUser1"),
                String::from("01-01-01")));
        logs.messages.push(
            Message::new_message(
                String::from("2"),
                String::from("InsertedUser2"),
                String::from("01-01-01")));
        write_logs(logs).await;
        let retrieved_logs = get_logs(String::from("01-01-01")).await;
        match retrieved_logs {
            Ok(_) => {
                assert_eq!(retrieved_logs.borrow().as_ref().unwrap().messages.len(), 2);
                for log in retrieved_logs.unwrap().messages {
                    println!("Log found: \n Message {} \n from User {} \n at timestamp {}",
                             log.message, log.user, log.timestamp);
                    assert_eq!(log.timestamp, "01-01-01");
                }
            }
            _ => println!("Message fetching did not work")
        }
    }
}
