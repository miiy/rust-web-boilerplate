use super::model::User;
use sqlx::mysql::MySqlPool;
use time::OffsetDateTime;

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

    pub async fn find_all(
        pool: &MySqlPool,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<User>, sqlx::Error> {
        let page = if page == 0 { 1 } else { page };
        let limit = page_size;
        let offset = (page - 1) * page_size;

        let users: Vec<User> = sqlx::query_as(
            "
        SELECT `id`, `name`, `email`
        FROM `users`
        WHERE `delete_time` IS NULL
        LIMIT ? OFFSET ?
        ",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(users)
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

    pub async fn soft_delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE users SET `delete_time` = ?
        WHERE `id`=? AND `delete_time` IS NULL
        ",
        )
        .bind(OffsetDateTime::now_utc())
        .bind(id)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
    }

    pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        DELETE FROM users
        WHERE `id`=?
        ",
        )
        .bind(id)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
    }
}
