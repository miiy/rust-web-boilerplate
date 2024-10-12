use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use rust_web::{config_api, config_app, db, error, middleware, AppState};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv
    dotenv().ok();

    // env_logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // db
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to create pool");

    // redis
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis = redis::Client::open(redis_url).expect("Failed to open redis");

    // actix web
    let app_name = env::var("APP_NAME").expect("APP_NAME must be set");
    let addrs = env::var("ADDRS").expect("ADDRS must be set");
    log::info!("starting HTTP server at {addrs}");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::cors())
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                app_name: app_name.clone(),
            }))
            .app_data(
                web::JsonConfig::default()
                    // register error_handler for JSON extractors.
                    .error_handler(error::json_error_handler),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis.clone()))
            .configure(config_app)
            .service(web::scope("/api").configure(config_api))
    })
    .bind(addrs)?
    .run()
    .await
}
