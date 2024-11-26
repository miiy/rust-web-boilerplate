use super::model::User;
use sqlx::mysql::MySqlPool;

impl User {
    pub async fn find_by_name(pool: &MySqlPool, name: &str) -> Result<Option<User>, sqlx::Error> {
        let item: Option<User> = sqlx::query_as(
            "
        SELECT `id`, `name`, `email`, `create_time`, `update_time`
        FROM `users`
        WHERE `name`=? AND `delete_time` IS NULL
        ",
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }
}
