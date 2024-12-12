use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use rust_web::{
    config, middleware, middleware::session, redis_session_store, server::route::config_app,
    AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let c = config::Config::new().expect("config error");

    // env_logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let (secret_key, session_store) = redis_session_store(&c).await;

    // actix web
    log::info!("Starting HTTP server at {}", c.server.addrs);
    let cookie_name = c.cookie.name.clone();
    let addrs = c.server.addrs.clone();
    HttpServer::new(move || {
        let app_state = AppState::new(&c);
        let shared_data = web::Data::new(app_state);
        App::new()
            .wrap(Logger::default())
            .wrap(session::session(
                session_store.clone(),
                cookie_name.clone(),
                secret_key.clone(),
            ))
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
