use super::handler;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // GET: /posts
    // GET: /posts_create
    // GET: /posts/{id}
    // GET: /posts/{id}/edit
    // GET: /posts/create (not support)
    cfg.service(
        web::scope("/posts")
            .service(web::resource("").route(web::get().to(handler::index)))
            .service(
                web::scope("/{id}")
                    .service(web::resource("").route(web::get().to(handler::detail)))
                    .service(web::resource("/edit").route(web::get().to(handler::edit))),
            ), // not support
               // .service(
               //     web::resource("/create").route(web::get().to(handler::create)),
               // ),
    )
    .service(web::resource("posts_create").route(web::get().to(handler::create)));
}
