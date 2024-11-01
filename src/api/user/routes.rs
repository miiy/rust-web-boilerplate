use super::handler;
use actix_web::web;

// GET /users
// GET /users/{id}
// POST /users
// PUT /users/{id}
// DELETE /users/{id}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/users").service(
            web::resource("/{id}")
                .route(web::get().to(handler::detail))
                .route(web::put().to(handler::update)),
        ),
    );
}
