use crate::client::auth::client::AuthClient;
use crate::client::post::client::PostClient;
use crate::template::Template;

pub mod client;
pub mod config;
pub mod error;
pub mod middleware;
pub mod pagination;
pub mod server;
pub mod template;
pub mod vite;

pub struct AppState {
    pub redis: redis::Client,
    pub template: Template,
    pub post_client: PostClient,
    pub auth_client: AuthClient,
}
