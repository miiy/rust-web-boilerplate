use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    // pub created_at: OffsetDateTime,
}
