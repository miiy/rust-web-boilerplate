use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn index(app_state: web::Data<AppState>) -> impl Responder {
    let app_name = &app_state.app_name;
    let mut ctx = tera::Context::new();
    ctx.insert("app_name", app_name);
    ctx.insert("page_title", "Home");
    ctx.insert("keywords", "keywords");
    ctx.insert("description", "description");
    let html = app_state.tera.render("index/index.html", &ctx).unwrap();
    HttpResponse::Ok().body(html)
}
