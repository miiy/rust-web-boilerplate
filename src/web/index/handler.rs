use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index/index.html")]
struct IndexTemplate<'a> {
    app_name: &'a str,
    page_title: &'a str,
    keywords: &'a str,
    description: &'a str,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let index = IndexTemplate {
        app_name: app_name,
        page_title: "Home",
        keywords: "keywords",
        description: "description",
    };
    HttpResponse::Ok().body(index.render().unwrap())
}
