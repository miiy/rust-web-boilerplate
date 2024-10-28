use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

// https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html#default
#[derive(Default, Debug, Serialize, Deserialize, FromRow)]
#[sqlx(default)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub create_time: Option<OffsetDateTime>,
    pub update_time: Option<OffsetDateTime>,
}
