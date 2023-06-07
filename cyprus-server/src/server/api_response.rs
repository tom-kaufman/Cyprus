use axum::{
    body::StreamBody,
    http::{
        self,
        header::{self, HeaderMap},
    },
    response,
};
use serde::Serialize;
use tokio_util::io::ReaderStream;

pub enum ApiResponse<T: Serialize> {
    Success(T),
    Error(http::StatusCode, String),
    File(tokio::fs::File, String),  // file object, file name
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
                (http::StatusCode::OK, response::Json::from(resp.message)).into_response()
            }
            ApiResponse::Error(status_code, message) => {
                let resp = ApiResponseBody {
                    status_code: status_code.as_u16(),
                    message,
                };
                (status_code, response::Json::from(resp.message)).into_response()
            }
            ApiResponse::File(file, filename) => {
                let stream = ReaderStream::new(file);
                let body = StreamBody::new(stream);

                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, "audio/mp4".parse().unwrap());
                headers.insert(
                    header::CONTENT_DISPOSITION,
                    format!(
                        "attachment; filename=\"{}\"",
                        filename
                    )
                    .parse()
                    .unwrap(),
                );

                (headers, body).into_response()
            }
        }
    }
}
