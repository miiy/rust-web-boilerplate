use serde::Serialize;

// index/index.html
#[derive(Serialize)]
pub struct IndexTemplate {
    pub hello: String,
}
