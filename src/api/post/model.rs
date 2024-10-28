use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Default, Debug, FromRow)]
#[sqlx(default)]
pub struct Post {
    pub id: u64,
    pub category_id: u64,
    pub title: String,
    pub author: String,
    pub content: String,
    pub create_time: Option<OffsetDateTime>,
    pub update_time: Option<OffsetDateTime>,
}
