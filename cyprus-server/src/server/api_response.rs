use axum::{http, response};
use serde::Serialize;

pub enum ApiResponse<T: Serialize> {
    Success(T),
    Error(http::StatusCode, String),
}

#[derive(Serialize)]
struct ApiResponseBody<T: Serialize> {
    status_code: u16,
    message: T,
}

impl<T> response::IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> response::Response {
        match self {
            ApiResponse::Success(message) => {
                let resp = ApiResponseBody {
                    status_code: http::StatusCode::OK.as_u16(),
                    message,
                };
                response::Json::from(resp).into_response()
            }
            ApiResponse::Error(status_code, message) => {
                let resp = ApiResponseBody {
                    status_code: status_code.as_u16(),
                    message,
                };
                response::Json::from(resp).into_response()
            }
        }
    }
}
