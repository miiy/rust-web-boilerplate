use super::handler;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // GET: /register
    // GET: /login
    cfg.service(web::resource("/register").route(web::get().to(handler::register)))
        .service(web::resource("/login").route(web::get().to(handler::login)))
        .service(web::resource("/logout").route(web::get().to(handler::logout)));
}
