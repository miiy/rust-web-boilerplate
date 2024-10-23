use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Post {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub content: String,
    // pub created_at: OffsetDateTime,
}
