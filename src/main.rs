use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use rust_web::{
    api::{errors, routes::config_api},
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

    // actix web
    log::info!("Starting HTTP server at {}", c.server.addrs);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::cors::cors())
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                app_name: c.app.name.clone(),
            }))
            .app_data(
                web::JsonConfig::default()
                    // register error_handler for JSON extractors.
                    .error_handler(errors::json_error_handler),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis.clone()))
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
