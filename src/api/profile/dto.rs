use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// profile

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub name: String,
    pub email: String,
    #[serde(with = "time::serde::iso8601")]
    pub create_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub update_time: OffsetDateTime,
}
