use crate::web::post::dto;
use serde::Serialize;

// post/index.html
#[derive(Serialize)]
pub struct IndexTemplate {
    pub page_title: String,
    pub keywords: String,
    pub description: String,
    pub lists: Vec<dto::Post>,
}

// post/detail.html
#[derive(Serialize)]
pub struct DetailTemplate {
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}

// post/create.html
#[derive(Serialize)]
pub struct CreateTemplate {
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}

// post/edit.html
#[derive(Serialize)]
pub struct EditTemplate {
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}
