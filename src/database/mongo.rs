use mongodb::{Client, Database, options::ClientOptions};
use std::env;

pub struct MongoDB {
    pub connection: Database,
}

impl MongoDB {
    pub async fn new() -> MongoDB {
        let username = env::var("DB_USER").unwrap();
        let password = env::var("DB_PASSWORD").unwrap();
        let host = env::var("DB_HOST").unwrap();
        let database = env::var("DB_DATABASE").unwrap();

        let conn_string = format!(
            "mongodb+srv://{}:{}@{}/{}",
            username, password, host, database
        );
        let client_options = ClientOptions::parse(conn_string).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        let database = client.database("greenie_mvp");

        Self {
            connection: database,
        }
    }
}
