use super::model::Post;
use sqlx::mysql::MySqlPool;
use time::OffsetDateTime;

impl Post {
    pub async fn create(pool: &MySqlPool, item: &Post) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        INSERT INTO posts (`title`, `content`, `create_time`, `update_time`)
        VALUES(?, ?)
        ",
        )
        .bind(&item.title)
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
        SELECT `id`, `title`, `content`, `create_time`, `update_time`
        FROM `posts`
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
    ) -> Result<Vec<Post>, sqlx::Error> {
        let page = if page == 0 { 1 } else { page };
        let limit = page_size;
        let offset = (page - 1) * page_size;

        let posts: Vec<Post> = sqlx::query_as(
            "
        SELECT `id`, `title`, `content`
        FROM `posts`
        WHERE `delete_time` IS NULL
        LIMIT ? OFFSET ?
        ",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    pub async fn update(pool: &MySqlPool, item: &Post) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        UPDATE posts SET `title` = ?, `content` = ?, `update_time` = ?
        WHERE `id`=?
        ",
        )
        .bind(&item.title)
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

    pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
        sqlx::query(
            "
        DELETE FROM posts
        WHERE `id`=?
        ",
        )
        .bind(id)
        .execute(pool)
        .await
        .map(|x| x.rows_affected())
    }
}
