use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaginationResponse {
    pub page: i64,
    pub limit: i64,
    pub total_records: i64,
    pub total_page: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaginationQueryParams {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResponse<T> {
    pub message: String,
    pub data: T,
}