use super::template::*;
use crate::web::template;
use crate::AppState;
use actix_web::{get, web, Error, HttpResponse};

#[get("/about")]
async fn index(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = IndexTemplate {
        about: "about".to_string(),
    };

    let html = template::render("about/index.html", &template, &app_state)?;
    Ok(HttpResponse::Ok().body(html))
}
