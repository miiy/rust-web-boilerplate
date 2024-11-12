use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/about")]
async fn index(app_state: web::Data<AppState>) -> impl Responder {
    let app_name = &app_state.app_name;
    let mut ctx = tera::Context::new();
    ctx.insert("app_name", app_name);
    ctx.insert("page_title", "About");
    ctx.insert("keywords", "keywords");
    ctx.insert("description", "description");
    let html = app_state.tera.render("about/index.html", &ctx).unwrap();
    HttpResponse::Ok().body(html)
}
