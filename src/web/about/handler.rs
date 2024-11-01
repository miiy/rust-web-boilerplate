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
async fn index(app_state: web::Data<AppState>) -> impl Responder {
    let app_name = &app_state.app_name;
    let t = AboutTemplate {
        app_name: app_name,
        page_title: "About",
        keywords: "keywords",
        description: "description",
    };
    HttpResponse::Ok().body(t.render().unwrap())
}
