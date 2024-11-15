use web::template::Template;

pub mod api;
pub mod config;
pub mod db;
pub mod middleware;
pub mod web;

pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub redis: redis::Client,
    pub template: Template,
}
