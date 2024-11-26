use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use rust_web::{
    api::{jwt::JWT, route::config_api},
    config, db, middleware,
    web::route::config_app,
    web::template::Template,
    web::vite,
    web::middleware::session,
    json_config,
    AppState,
};
use std::str::FromStr;

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

    // jwt
    let jwt = JWT::new(c.jwt.secret.clone(), c.jwt.expires_in);

    // session
    // cookie secret key
    let secret_key = session::SecretKey::from_str(&c.cookie.secret_key)
        .expect("cookie secret_key error.");
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

    // actix web
    log::info!("Starting HTTP server at {}", c.server.addrs);
    HttpServer::new(move || {
        let shared_data = web::Data::new(AppState {
            db: pool.clone(),
            redis: redis.clone(),
            template: template.clone(),
            jwt: jwt.clone(),
        });

        App::new()
            .wrap(middleware::cors::cors(&c.app.url))
            .wrap(Logger::default())
            .wrap(session::session(
                session_store.clone(),
                c.cookie.name.clone(),
                secret_key.clone(),
            ))
            .app_data(shared_data)
            .app_data(json_config())
            // .service(
            //     web::scope("")
            //         .wrap(middleware::error::error_handlers())
            //         .configure(config_app),
            // )
            .configure(config_app)
            .service(web::scope("/api").configure(config_api))
            .service(fs::Files::new("/static", "./frontend/dist").use_last_modified(true))
            .service(web::resource("/favicon.ico").route(
                web::get().to(|| async { fs::NamedFile::open("./frontend/dist/favicon.ico") }),
            ))
    })
    .bind(&c.server.addrs)?
    .run()
    .await
}
