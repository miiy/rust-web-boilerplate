use serde::Serialize;

// about/index.html
pub const INDEX_TEMPLATE_PATH: &str = "about/index.html";

#[derive(Serialize)]
pub struct IndexTemplate {
    pub about: String,
}
