use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub mongo_collection: String,
    pub bind_addr: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            mongo_uri: env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
            mongo_db: env::var("MONGO_DB").unwrap_or_else(|_| "price_checker".to_string()),
            mongo_collection: env::var("MONGO_COLLECTION").unwrap_or_else(|_| "offers".to_string()),
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
        }
    }
}
