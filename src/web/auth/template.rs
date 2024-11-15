use serde::Serialize;

pub const REGISTER_TEMPLATE_PATH: &str = "auth/register.html";
pub const LOGIN_TEMPLATE_PATH: &str = "auth/login.html";

#[derive(Serialize)]
// auth/register.html
pub struct RegisterTemplate {
}

#[derive(Serialize)]
// auth/login.html
pub struct LoginTemplate {
}
