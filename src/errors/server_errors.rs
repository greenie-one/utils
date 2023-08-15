use mongodb::bson::document::ValueAccessError;

pub type ServerResult<T> = std::result::Result<T, ServerError>;
#[derive(Debug)]
pub enum ServerError {
    AzureError(String),

    MongoDBError(String),
    MongoValueAccessError(String),
}

impl From<azure_core::Error> for ServerError {
    fn from(value: azure_core::Error) -> Self {
        ServerError::AzureError(format!("Azure Core Error: {:?}", value))
    }
}

impl From<mongodb::error::Error> for ServerError {
    fn from(value: mongodb::error::Error) -> Self {
        ServerError::MongoDBError(format!("MongoDB Error: {:?}", value))
    }
}

impl From<ValueAccessError> for ServerError {
    fn from(value: ValueAccessError) -> Self {
        ServerError::MongoValueAccessError(format!("MongoDB Error: {:?}", value))
    }
}

impl ToString for ServerError {
    fn to_string(&self) -> String {
        match self {
            ServerError::AzureError(e) => format!("Azure Error: {}", e),
            ServerError::MongoDBError(e) => format!("MongoDB Error: {}", e),
            ServerError::MongoValueAccessError(e) => format!("MongoDB Value Access Error: {}", e),
        }
    }
}