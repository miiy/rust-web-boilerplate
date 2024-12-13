use actix_files as fs;
use actix_identity::IdentityMiddleware;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use rust_web::{config, middleware, middleware::session, server::route::config_app, AppState};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let c = config::Config::new().expect("config error");

    // env_logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // session store
    let session_store = session::redis_store(c.redis.url.clone())
        .await
        .expect("Failed to open redis");

    // actix web
    log::info!("Starting HTTP server at {}", c.server.addrs);
    let addrs = c.server.addrs.clone();
    HttpServer::new(move || {
        // session
        let session_mw = session::session(session_store.clone(), c.session.clone());
        // identity
        let expiration = Duration::from_secs(c.session.expiration);
        let identity_mw = IdentityMiddleware::builder()
            .visit_deadline(Some(expiration))
            .build();

        let app_state = AppState::new(&c);
        let shared_data = web::Data::new(app_state);

        App::new()
            .wrap(Logger::default())
            // Install the identity framework first.
            .wrap(identity_mw)
            // The identity system is built on top of sessions. You must install the session
            // middleware to leverage `actix-identity`. The session middleware must be mounted
            // AFTER the identity middleware: `actix-web` invokes middleware in the OPPOSITE
            // order of registration when it receives an incoming request.
            .wrap(session_mw)
            .app_data(shared_data)
            .service(fs::Files::new("/static", "./frontend/dist").use_last_modified(true))
            .service(web::resource("/favicon.ico").route(
                web::get().to(|| async { fs::NamedFile::open("./frontend/dist/favicon.ico") }),
            ))
            .service(
                web::scope("")
                    .wrap(middleware::error_handlers::error_handlers())
                    .configure(config_app),
            )
    })
    .bind(&addrs)?
    .run()
    .await
}
