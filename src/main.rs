use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use rust_web::{
    api::{error, routes::config_api},
    config, db, middleware,
    web::routes::config_app,
    AppState,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let c = config::Config::new().expect("config error");

    // env_logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // db
    let pool = db::init_pool(&c.database.url)
        .await
        .expect("Failed to create pool");

    // redis
    let redis = redis::Client::open(c.redis.url.clone()).expect("Failed to open redis");

    // session store
    let session_store = middleware::session::redis_store(c.redis.url.clone())
        .await
        .expect("Failed to open redis");

    // actix web
    log::info!("Starting HTTP server at {}", c.server.addrs);
    HttpServer::new(move || {
        let shared_data = web::Data::new(AppState {
            app_name: c.app.name.clone(),
            db: pool.clone(),
            redis: redis.clone(),
        });

        App::new()
            .wrap(middleware::cors::cors(&c.app.url))
            .wrap(Logger::default())
            .wrap(middleware::session::session(session_store.clone()))
            .app_data(shared_data)
            .app_data(
                web::JsonConfig::default()
                    // register error_handler for JSON extractors.
                    .error_handler(error::json_error_handler),
            )
            .configure(config_app)
            .service(web::scope("/api").configure(config_api))
            .service(fs::Files::new("/static", "./static").use_last_modified(true))
            .service(
                web::resource("/favicon.ico")
                    .route(web::get().to(|| async { fs::NamedFile::open("./static/favicon.ico") })),
            )
    })
    .bind(&c.server.addrs)?
    .run()
    .await
}
