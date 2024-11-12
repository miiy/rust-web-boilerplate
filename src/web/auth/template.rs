use serde::Serialize;

#[derive(Serialize)]
// auth/register.html
pub struct RegisterTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}

#[derive(Serialize)]
// auth/login.html
pub struct LoginTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}
