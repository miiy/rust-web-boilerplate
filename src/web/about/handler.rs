use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "about/index.html")]
struct AboutTemplate<'a> {
    app_name: &'a str,
    page_title: &'a str,
    keywords: &'a str,
    description: &'a str,
}

#[get("/about")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    let tmpl = AboutTemplate {
        app_name: app_name,
        page_title: "About",
        keywords: "keywords",
        description: "description",
    };
    HttpResponse::Ok().body(tmpl.render().unwrap())
}
