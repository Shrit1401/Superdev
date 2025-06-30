use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response<T> {
    Success(SuccessResponse<T>),
    Error(ErrorResponse),
}

pub fn success<T: serde::Serialize>(data: T) -> axum::Json<Response<T>> {
    axum::Json(Response::Success(SuccessResponse {
        success: true,
        data,
    }))
}

pub fn error<T>(message: &str) -> axum::Json<Response<T>> {
    axum::Json(Response::Error(ErrorResponse {
        success: false,
        error: message.to_string(),
    }))
}
