use crate::error::APIError;

pub struct Service;

impl Service {
    pub async fn health(redis: &redis::Client) -> Result<(), APIError> {
        let mut con = redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("{e}");
                APIError::InternalError(0)
            })?;

        let ret = redis::cmd("PING")
            .query_async::<String>(&mut con)
            .await
            .map_err(|e| {
                log::error!("{e}");
                APIError::InternalError(0)
            })?;
        log::info!("{}", ret);

        Ok(())
    }
}
