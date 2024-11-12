use serde::Serialize;

#[derive(Serialize)]
// about/index.html
pub struct IndexTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}
