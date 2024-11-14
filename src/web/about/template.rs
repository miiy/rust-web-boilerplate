use serde::Serialize;

// about/index.html
#[derive(Serialize)]
pub struct IndexTemplate {
    pub about: String,
}
