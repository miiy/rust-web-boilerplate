use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// list
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub lists: Vec<ListResponseItem>,
}
#[derive(Debug, Serialize)]
pub struct ListResponseItem {
    pub name: String,
    pub email: String,
    pub create_time: NaiveDateTime,
}

// detail
#[derive(Debug, Serialize, Deserialize)]
pub struct DetailRequest {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailResponse {
    pub name: String,
    pub email: String,
    pub create_time: NaiveDateTime,
}

// create
#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub id: u64,
}

// update
#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateResponse {
    pub id: u64,
}

// delete
#[derive(Debug, Serialize)]
pub struct DeleteResponse;
