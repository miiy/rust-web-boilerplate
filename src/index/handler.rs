use askama::Template;
use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;

#[derive(Template)]
#[template(path = "index/index.html")]
struct IndexTemplate<'a> {
    app_name: &'a str,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let index = IndexTemplate { app_name: app_name };
    HttpResponse::Ok().body(index.render().unwrap())
}
