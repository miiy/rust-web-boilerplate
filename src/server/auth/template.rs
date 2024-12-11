use serde::Serialize;

pub const REGISTER_TEMPLATE_PATH: &str = "auth/register.html";
pub const REGISTER_RESOURCE_NAME: &str = "src/register.js";

pub const LOGIN_TEMPLATE_PATH: &str = "auth/login.html";
pub const LOGIN_RESOURCE_NAME: &str = "src/login.js";

#[derive(Serialize)]
// auth/register.html
pub struct RegisterTemplate {}

#[derive(Serialize)]
// auth/login.html
pub struct LoginTemplate {}
