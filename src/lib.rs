pub mod db;
pub mod config;
pub mod api;
pub mod web;
pub mod middleware;

pub struct AppState {
    pub app_name: String,
}
