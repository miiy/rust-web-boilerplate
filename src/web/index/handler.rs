use super::template::*;
use crate::AppState;
use actix_web::{get, web, Error, HttpResponse};
use serde_json::json;

#[get("/")]
async fn index(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = IndexTemplate {
        hello: "Hello, world!".to_string(),
    };
    let html = app_state
        .template
        .render_with(INDEX_TEMPLATE_PATH, INDEX_RESOURCE_NAME, &template, |ctx| {
            ctx.insert(
                "metadata",
                &json!({
                    "title": "Home",
                    "description": "description",
                    "keywords": "keywords",
                }),
            );
        })?;
    Ok(HttpResponse::Ok().body(html))
}
