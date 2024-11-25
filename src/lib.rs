use web::template::Template;
use api::jwt::JWT;

pub mod api;
pub mod config;
pub mod db;
pub mod middleware;
pub mod web;

pub struct AppState {
    pub db: sqlx::MySqlPool,
    pub redis: redis::Client,
    pub template: Template,
    pub jwt: JWT,
}
