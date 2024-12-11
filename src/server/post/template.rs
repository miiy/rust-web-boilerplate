use serde::Serialize;

pub const INDEX_TEMPLATE_PATH: &str = "post/index.html";
pub const INDEX_RESOURCE_NAME: &str = "src/base.js";

pub const DETAIL_TEMPLATE_PATH: &str = "post/detail.html";
pub const DETAIL_RESOURCE_NAME: &str = "src/base.js";

pub const CREATE_TEMPLATE_PATH: &str = "post/create.html";
pub const CREATE_RESOURCE_NAME: &str = "src/base.js";

pub const EDIT_TEMPLATE_PATH: &str = "post/edit.html";
pub const EDIT_RESOURCE_NAME: &str = "src/base.js";

use super::dto::Post;

// post/index.html
#[derive(Serialize)]
pub struct IndexTemplate {
    pub data: Vec<Post>,
    pub pagination: String,
}

// post/detail.html
#[derive(Serialize)]
pub struct DetailTemplate {
    pub post: Post,
}

// post/create.html
#[derive(Serialize)]
pub struct CreateTemplate {}

// post/edit.html
#[derive(Serialize)]
pub struct EditTemplate {}
