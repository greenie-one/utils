use redis::{Client};
use std::env;

pub fn get_client() -> Client {
    let username = env::var("REDIS_USERNAME").unwrap();
    let password = env::var("REDIS_PASSWORD").unwrap();
    let host = env::var("REDIS_HOST").unwrap();
    let port = env::var("REDIS_PORT").unwrap();
    let database = env::var("REDIS_DB").unwrap();

    let conn_string = format!(
        "redis://{}:{}@{}:{}/{}",
        username, password, host, port, database
    );
    redis::Client::open(conn_string).unwrap()
}
