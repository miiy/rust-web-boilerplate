use super::handler;
use actix_web::web;

// POST /auth/register
// POST /auth/login
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/auth")
            .service(web::resource("/register").route(web::post().to(handler::register)))
            .service(web::resource("/login").route(web::post().to(handler::login))),
    );
}
