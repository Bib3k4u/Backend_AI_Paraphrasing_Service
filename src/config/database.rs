use dotenv::dotenv;
use mongodb::{Client, Database};
use std::env;

pub async fn init_database() -> Database {
    dotenv().ok();

    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to connect to MongoDB");

    let db_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    client.database(&db_name)
}
