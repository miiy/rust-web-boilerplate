use derive_more::derive::Display;

pub struct Service;
#[derive(Display)]
pub struct HealthError;

impl Service {
    pub async fn health(redis: &redis::Client) -> Result<(), HealthError> {
        let mut con = redis
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| {
                log::error!("{e}");
                HealthError{}
            })?;

        let ret = redis::cmd("PING")
            .query_async::<String>(&mut con)
            .await
            .map_err(|e| {
                log::error!("{e}");
                HealthError{}
            })?;
        log::info!("{}", ret);

        Ok(())
    }
}
