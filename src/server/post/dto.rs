use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u64,
    pub category_id: u64,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct IndexRequest {
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct IndexResponse {
    pub total_pages: u32,
    pub data: Vec<Post>,
}

#[derive(Debug, Deserialize)]
pub struct DetailRequest {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct DetailResponse {
    pub post: Post,
}

#[derive(Debug, Deserialize)]
pub struct CreateRequest {}

#[derive(Debug, Serialize)]
pub struct CreateResponse {}

#[derive(Debug, Deserialize)]
pub struct EditRequest {
    pub id: u64,
}

#[derive(Debug, Serialize)]
pub struct EditResponse {
    pub post: Post,
}
