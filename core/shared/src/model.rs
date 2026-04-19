use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
    pub product_title: String,
    pub shop_name: String,
    pub price: String,
    pub link: String,
    pub timestamp: String,
    pub normalized_query: String,
    pub keywords: Vec<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    pub from_cache: bool,
    pub offers: Vec<Offer>,
}
