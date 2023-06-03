use mongodb::{options::ClientOptions, Client};

pub struct Database {}

impl Database {
    pub async fn get_client() -> mongodb::error::Result<Client>{
        let config = DatabaseConfig::new();
        let mut client_options = ClientOptions::parse(&config.uri).await?;

        client_options.min_pool_size = config.min_pool_size;
        client_options.max_pool_size = config.max_pool_size;

        let client = Client::with_options(client_options)?;

        Ok(client)
    }
}

pub struct DatabaseConfig {
    pub uri: String,
    pub min_pool_size: Option<u32>,
    pub max_pool_size: Option<u32>,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        let mongo_uri: String = std::env::var("MONGO_URI")
            .expect("Failed to load `MONGO_MAX_POOL_SIZE` environment variable.");

        let mongo_min_pool_size: u32 = std::env::var("MONGO_MIN_POOL_SIZE")
            .expect("Failed to load `MONGO_MIN_POOL_SIZE` environment variable.")
            .parse()
            .expect("Failed to parse `MONGO_MIN_POOL_SIZE` environment variable.");

        let mongo_max_pool_size: u32 = std::env::var("MONGO_MAX_POOL_SIZE")
            .expect("Failed to load `MONGO_MAX_POOL_SIZE` environment variable.")
            .parse()
            .expect("Failed to parse `MONGO_MAX_POOL_SIZE` environment variable.");

        Self {
            uri: mongo_uri,
            min_pool_size: Some(mongo_min_pool_size),
            max_pool_size: Some(mongo_max_pool_size),
        }
    }
}
