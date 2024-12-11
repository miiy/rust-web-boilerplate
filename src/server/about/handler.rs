use super::template::*;
use crate::AppState;
use actix_web::{get, web, Error, HttpResponse};

#[get("/about")]
async fn index(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = IndexTemplate {
        about: "about".to_string(),
    };

    let html = app_state
        .template
        .render(INDEX_TEMPLATE_PATH, INDEX_RESOURCE_NAME, &template)?;
    Ok(HttpResponse::Ok().body(html))
}
