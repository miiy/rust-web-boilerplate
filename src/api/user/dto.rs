use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// list
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub lists: Vec<ListResponseItem>,
}
#[derive(Debug, Serialize)]
pub struct ListResponseItem {
    pub name: String,
    pub email: String,
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
    #[serde(with = "time::serde::iso8601")]
    pub create_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub update_time: OffsetDateTime,
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
// pub struct DeleteResponse;
// json: null
//
// pub struct DeleteResponse{}
// json: {}
#[derive(Debug, Serialize)]
pub struct DeleteResponse {}
