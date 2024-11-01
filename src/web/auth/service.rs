use crate::web::auth::template::{LoginTemplate, RegisterTemplate};
use crate::AppState;
use actix_web::web;
use std::fmt::Error;

pub async fn register(app_state: web::Data<AppState>) -> RegisterTemplate {
    let app_name = &app_state.app_name;
    RegisterTemplate {
        app_name: app_name.to_string(),
        page_title: "Register".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    }
}

pub async fn login(app_state: web::Data<AppState>) -> LoginTemplate {
    let app_name = &app_state.app_name;
    LoginTemplate {
        app_name: app_name.to_string(),
        page_title: "Login".to_string(),
        keywords: "keywords".to_string(),
        description: "description".to_string(),
    }
}

pub async fn logout(app_state: web::Data<AppState>) -> Result<(), Error> {
    let _app_name = &app_state.app_name;
    Ok(())
}
