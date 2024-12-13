use crate::{
    client::{auth::client::AuthClient, post::client::PostClient},
    config::Config,
    template::Template,
    vite::Manifest,
};
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

impl AppState {
    pub fn new(c: &Config) -> Self {
        // redis
        let redis = redis::Client::open(c.redis.url.clone()).expect("Failed to open redis");

        // template
        let metadata = c.app.metadata.clone();
        let mut template =
            Template::new("templates/**/*.html", metadata.into()).expect("template error");

        // manifest
        let manifest =
            Manifest::new("./frontend/dist/.vite/manifest.json").expect("Failed to parse manifest");
        template.register_function("manifest", vite::make_manifest(manifest.clone()));
        template.register_function(
            "imported_chunks",
            vite::make_imported_chunks(manifest.clone()),
        );

        // post client
        let post_client = PostClient::new(c.post_client.addrs.clone());

        // auth client
        let auth_client = AuthClient::new(c.auth_client.addrs.clone());

        Self {
            redis: redis,
            template: template,
            post_client: post_client,
            auth_client: auth_client,
        }
    }
}
