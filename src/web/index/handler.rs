use super::template::*;
use crate::web::template;
use crate::AppState;
use actix_web::{get, web, Error, HttpResponse};
use serde_json::json;

#[get("/")]
async fn index(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let template = IndexTemplate {
        hello: "Hello, world!".to_string(),
    };
    let html = template::render_with("index/index.html", &template, &app_state, |ctx| {
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
