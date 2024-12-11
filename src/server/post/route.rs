use super::handler;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    // GET: /posts
    // GET: /posts?page=1
    // GET: /posts/pages/{page}
    // GET: /posts/create (notice: route sorting is important)
    // GET: /posts/{id}
    // GET: /posts/{id}/edit
    cfg.service(
        web::scope("/posts")
            .service(web::resource("").route(web::get().to(handler::index)))
            .service(web::resource("/pages/{page}").route(web::get().to(handler::index)))
            .service(web::resource("/create").route(web::get().to(handler::create)))
            .service(
                web::scope("/{id}")
                    .service(web::resource("").route(web::get().to(handler::detail)))
                    .service(web::resource("/edit").route(web::get().to(handler::edit))),
            ),
    )
    .service(web::resource("posts_create").route(web::get().to(handler::create)));
}
