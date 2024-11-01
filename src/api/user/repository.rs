use super::model::User;
use sqlx::mysql::MySqlPool;

impl User {
    pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<User>, sqlx::Error> {
        let item: Option<User> = sqlx::query_as(
            "
        SELECT `id`, `name`, `email`, `create_time`, `update_time`
        FROM `users`
        WHERE `id`=? AND `delete_time` IS NULL
        ",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }

    pub async fn update(pool: &MySqlPool, user: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE users SET `name` = ?, `email` = ?, `update_time` = ?
        WHERE `id`=?
        ",
        )
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.update_time)
        .bind(&user.id)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
    }
}
