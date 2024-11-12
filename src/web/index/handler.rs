use super::template::IndexTemplate;
use crate::web::error::AppError;
use crate::AppState;
use actix_web::{get, web, Error, HttpResponse};

#[get("/")]
async fn index(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = IndexTemplate {
        app_name: app_state.app_name.clone(),
        page_title: "Home".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    };
    let html = app_state
        .tera
        .render(
            "index/index.html",
            &tera::Context::from_serialize(&template).map_err(|e| AppError::from(e))?,
        )
        .map_err(|e| AppError::from(e))?;
    Ok(HttpResponse::Ok().body(html))
}
