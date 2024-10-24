use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Default, Debug, FromRow)]
#[sqlx(default)]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub content: String,
    pub create_time: Option<OffsetDateTime>,
    pub update_time: Option<OffsetDateTime>,
}
