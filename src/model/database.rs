extern crate mongodb;

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
