pub mod api;
pub mod config;
pub mod db;
pub mod middleware;
pub mod web;

pub struct AppState {
    pub app_name: String,
    pub db: sqlx::MySqlPool,
    pub redis: redis::Client,
}
