use serde::Serialize;

#[derive(Serialize)]
pub struct IndexTemplate {
    pub hello: String,
}

pub const INDEX_TEMPLATE_PATH: &str = "index/index.html";
