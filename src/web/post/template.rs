use crate::web::post::dto;
use askama::Template;

#[derive(Template)]
#[template(path = "post/index.html")]
pub struct IndexTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
    pub lists: Vec<dto::Post>,
}

#[derive(Template)]
#[template(path = "post/detail.html")]
pub struct DetailTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}

#[derive(Template)]
#[template(path = "post/create.html")]
pub struct CreateTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}

#[derive(Template)]
#[template(path = "post/edit.html")]
pub struct EditTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}
