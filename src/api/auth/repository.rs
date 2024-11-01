use super::model::User;
use sqlx::MySqlPool;

impl User {
    pub async fn create(pool: &MySqlPool, item: &User) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        INSERT INTO users (`name`, `email`, `create_time`, `update_time`)
        VALUES(?, ?, ?, ?)
        ",
        )
        .bind(&item.name)
        .bind(&item.email)
        .bind(&item.create_time)
        .bind(&item.update_time)
        .execute(pool)
        .await
        .map(|x| x.last_insert_id())
    }

    pub async fn find(
        pool: &MySqlPool,
        name: String,
        password: String,
    ) -> Result<Option<User>, sqlx::Error> {
        let item: Option<User> = sqlx::query_as(
            "
        SELECT `id`, `name`, `email`, `create_time`, `update_time`
        FROM `users`
        WHERE `name`=? AND `password`=? AND `delete_time` IS NULL
        ",
        )
        .bind(name)
        .bind(password)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }
}
