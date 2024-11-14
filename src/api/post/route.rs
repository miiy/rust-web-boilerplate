use super::handler;
use actix_web::web;

// GET /posts
// GET /posts/{id}
// POST /posts
// PUT /posts/{id}
// DELETE /posts/{id}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1/posts")
            .service(
                web::resource("")
                    .route(web::get().to(handler::list))
                    .route(web::post().to(handler::create)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(handler::detail))
                    .route(web::put().to(handler::update))
                    .route(web::delete().to(handler::delete)),
            ),
    );
}
