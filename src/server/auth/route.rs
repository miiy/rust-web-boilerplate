use super::handler;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // GET: /register
    // GET: /login
    // POST: /api/register
    // POST: /api/login
    cfg.service(web::resource("/register").route(web::get().to(handler::show_register)))
        .service(web::resource("/api/register").route(web::post().to(handler::register)))
        .service(web::resource("/login").route(web::get().to(handler::show_login)))
        .service(web::resource("/api/login").route(web::post().to(handler::login)))
        .service(web::resource("/profile").route(web::get().to(handler::profile)))
        .service(web::resource("/logout").route(web::post().to(handler::logout)));
}
