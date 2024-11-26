use super::handler;
use actix_web::web;

// GET /users/{username}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users")
            .service(web::resource("/{username}").route(web::get().to(handler::user))),
    );
}
