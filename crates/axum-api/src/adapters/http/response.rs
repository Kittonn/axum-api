use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct ApiSuccessResponse<T> {
    pub data: T,
}

impl<T: Serialize> ApiSuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}
