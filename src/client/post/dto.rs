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
pub struct ListResponse {
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub data: Vec<Post>,
}

#[derive(Debug, Deserialize)]
pub struct DetailResponse {
    pub id: u64,
    pub category_id: u64,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}
