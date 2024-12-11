use crate::error::AppError;

pub struct Service;

impl Service {
    pub async fn health(redis: &redis::Client) -> Result<(), AppError> {
        let mut con = redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::InternalServerError {
                source: Box::new(e),
            })?;

        let ret = redis::cmd("PING")
            .query_async::<String>(&mut con)
            .await
            .map_err(|e| AppError::InternalServerError {
                source: Box::new(e),
            })?;
        log::info!("{}", ret);

        Ok(())
    }
}
