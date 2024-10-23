use serde::{Deserialize, Serialize};

// list
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub lists: Vec<ListResponseItem>,
}
#[derive(Debug, Serialize)]
pub struct ListResponseItem {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub content: String,
}

// detail
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailResponse {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub content: String,
}

// create
#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub user_id: u64,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub id: u64,
}

// update
#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateResponse {
    pub id: u64,
}

// delete
#[derive(Debug, Serialize)]
pub struct DeleteResponse;
