use config::AppMetaData;
use tera::Tera;

pub mod api;
pub mod config;
pub mod db;
pub mod middleware;
pub mod web;

pub struct AppState {
    pub metadata: AppMetaData,
    pub db: sqlx::MySqlPool,
    pub redis: redis::Client,
    pub tera: Tera,
}
