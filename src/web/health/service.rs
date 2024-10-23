use crate::web::errors::ServiceError;

pub struct Service;

impl Service {
    pub async fn health(redis: &redis::Client) -> Result<(), ServiceError> {
        let mut con = redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("{e}");
                ServiceError::InternalServerError
            })?;

        let ret = redis::cmd("PING")
            .query_async::<String>(&mut con)
            .await
            .map_err(|e| {
                log::error!("{e}");
                ServiceError::InternalServerError
            })?;
        log::info!("{}", ret);

        Ok(())
    }
}
