use super::handler;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // domain: /posts/{id}
    // domain: /posts/{id}/edit
    cfg.service(
        web::scope("/posts")
            .service(web::resource("").route(web::get().to(handler::index)))
            .service(
                web::scope("/{id}")
                    .service(web::resource("").route(web::get().to(handler::detail)))
                    .service(web::resource("/edit").route(web::get().to(handler::edit))),
            )
            .service(
                // todo
                web::scope("/create")
                    .service(web::resource("").route(web::get().to(handler::create))),
            ),
    );
}
