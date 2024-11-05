use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// me

#[derive(Debug, Serialize, Deserialize)]
pub struct MeResponse {
    pub name: String,
    pub email: String,
    #[serde(with = "time::serde::iso8601")]
    pub create_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub update_time: OffsetDateTime,
}
