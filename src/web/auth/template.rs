use askama::Template;

#[derive(Template)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {
    pub app_name: String,
    pub page_title: String,
    pub keywords: String,
    pub description: String,
}
