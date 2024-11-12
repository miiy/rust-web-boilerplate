use serde::Serialize;

// index/index.html
#[derive(Serialize)]
pub struct IndexTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}
