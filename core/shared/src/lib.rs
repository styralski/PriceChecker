pub mod model;
pub mod normalize;

pub use model::{Offer, SearchRequest, SearchResponse};
pub use normalize::{normalize_query, split_keywords};
