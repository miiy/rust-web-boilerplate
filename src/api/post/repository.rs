use super::model::Post;
use sqlx::mysql::MySqlPool;
use time::OffsetDateTime;

impl Post {
    pub async fn create(pool: &MySqlPool, item: &Post) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        INSERT INTO posts (`category_id`, `title`, `author`, `content`, `create_time`, `update_time`)
        VALUES(?, ?, ?, ?, ?, ?)
        ",
        )
        .bind(&item.category_id)
            .bind(&item.title)
            .bind(&item.author)
        .bind(&item.content)
        .bind(&item.create_time)
        .bind(&item.update_time)
        .execute(pool)
        .await
        .map(|x| x.last_insert_id())
    }

    pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Post>, sqlx::Error> {
        let item: Option<Post> = sqlx::query_as(
            "
        SELECT `id`, `category_id`, `title`,`author`, `content`, `create_time`, `update_time`
        FROM `posts`
        WHERE `id`=? AND `delete_time` IS NULL
        ",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(item)
    }

    pub async fn find_count(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE delete_time IS NULL")
        .fetch_one(pool)
        .await?;
        Ok(count)
    }

    pub async fn find_all(
        pool: &MySqlPool,
        limit: u32,
        offset: u64,
    ) -> Result<Vec<Post>, sqlx::Error> {
        let items: Vec<Post> = sqlx::query_as(
            "
        SELECT `id`, `category_id`, `title`, `author`, `content`
        FROM `posts`
        WHERE `delete_time` IS NULL
        ORDER BY `create_time` DESC
        LIMIT ? OFFSET ?
        ",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(items)
    }

    pub async fn update(pool: &MySqlPool, item: &Post) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE posts SET `category_id` = ?, `title` = ?, `author` = ?, `content` = ?, `update_time` = ?
        WHERE `id`=?
        ",
        )
            .bind(&item.category_id)
            .bind(&item.title)
            .bind(&item.author)
            .bind(&item.content)
            .bind(&item.update_time)
            .bind(&item.id)
            .execute(pool)
            .await
            .map(|x| x.rows_affected())
    }

    pub async fn soft_delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE posts SET `delete_time` = ?
        WHERE `id`=? AND `delete_time` IS NULL
        ",
        )
        .bind(OffsetDateTime::now_utc())
        .bind(id)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
    }
}
