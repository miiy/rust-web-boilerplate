use super::handler;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let v1 = web::scope("/v1")
        .service(handler::list)
        .service(handler::detail)
        .service(handler::create)
        .service(handler::update)
        .service(handler::delete);
    cfg.service(v1);
}
