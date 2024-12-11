use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use rust_web::{
    client::auth::client::AuthClient, client::post::client::PostClient, config, middleware,
    middleware::session, server::route::config_app, template::Template, vite, AppState,
};
use std::str::FromStr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let c = config::Config::new().expect("config error");

    // env_logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // redis
    let redis = redis::Client::open(c.redis.url.clone()).expect("Failed to open redis");

    // session
    // cookie secret key
    let secret_key =
        session::SecretKey::from_str(&c.cookie.secret_key).expect("cookie secret_key error.");
    // session store
    let session_store = session::redis_store(c.redis.url.clone())
        .await
        .expect("Failed to open redis");

    // template
    let mut template =
        Template::new("templates/**/*.html", c.app.metadata.into()).expect("template error");

    // manifest
    let manifest = vite::Manifest::new("./frontend/dist/.vite/manifest.json")
        .expect("Failed to parse manifest");
    template.register_function("manifest", vite::make_manifest(manifest.clone()));
    template.register_function(
        "imported_chunks",
        vite::make_imported_chunks(manifest.clone()),
    );

    // post client
    let post_client = PostClient::new(c.post_client.addrs.clone());

    // auth client
    let auth_client = AuthClient::new(c.auth_client.addrs.clone());

    // actix web
    log::info!("Starting HTTP server at {}", c.server.addrs);
    HttpServer::new(move || {
        let shared_data = web::Data::new(AppState {
            redis: redis.clone(),
            template: template.clone(),
            post_client: post_client.clone(),
            auth_client: auth_client.clone(),
        });

        App::new()
            .wrap(Logger::default())
            .wrap(session::session(
                session_store.clone(),
                c.cookie.name.clone(),
                secret_key.clone(),
            ))
            .app_data(shared_data)
            // .service(
            //     web::scope("")
            //         .wrap(middleware::error_handler::error_handlers())
            //         .configure(config_app),
            // )
            .configure(config_app)
            .service(fs::Files::new("/static", "./frontend/dist").use_last_modified(true))
            .service(web::resource("/favicon.ico").route(
                web::get().to(|| async { fs::NamedFile::open("./frontend/dist/favicon.ico") }),
            ))
    })
    .bind(&c.server.addrs)?
    .run()
    .await
}
