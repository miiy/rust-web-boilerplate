use crate::web::post::dto;
use serde::Serialize;

pub const INDEX_TEMPLATE_PATH: &str = "post/index.html";
pub const DETAIL_TEMPLATE_PATH: &str = "post/detail.html";
pub const CREATE_TEMPLATE_PATH: &str = "post/create.html";
pub const EDIT_TEMPLATE_PATH: &str = "post/edit.html";

// post/index.html
#[derive(Serialize)]
pub struct IndexTemplate {
    pub lists: Vec<dto::Post>,
}

// post/detail.html
#[derive(Serialize)]
pub struct DetailTemplate {
}

// post/create.html
#[derive(Serialize)]
pub struct CreateTemplate {
}

// post/edit.html
#[derive(Serialize)]
pub struct EditTemplate {
}
