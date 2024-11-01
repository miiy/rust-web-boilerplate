use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// detail
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailResponse {
    pub name: String,
    pub email: String,
    #[serde(with = "time::serde::iso8601")]
    pub create_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub update_time: OffsetDateTime,
}

// update
#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateResponse {
    pub id: u64,
}
