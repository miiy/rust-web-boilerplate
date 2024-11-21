use serde::Serialize;

// about/index.html
pub const INDEX_TEMPLATE_PATH: &str = "about/index.html";
pub const INDEX_RESOURCE_NAME: &str = "src/about.js";

#[derive(Serialize)]
pub struct IndexTemplate {
    pub about: String,
}
