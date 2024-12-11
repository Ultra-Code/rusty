use std::{backtrace::BacktraceStatus, error::Error};

use actix_web::{
    App, HttpResponse, error, get,
    http::{StatusCode, header::ContentType},
};
use derive_more::derive::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display("An internal error occurred. Please try again later.")]
    InternalError,

    #[display("bad request")]
    BadClientData,

    #[display("timeout")]
    Timeout,

    #[display("Validation error on field: {field}")]
    ValidationError { field: String },
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}

#[get("/errors")]
async fn index() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}

#[get("/helper")]
async fn helper() -> actix_web::Result<String> {
    use log::{error, info};
    info!(
        "Info from error module would not show because log level is set to error"
    );
    error!("Error in /helper will show");
    let result = Err(MyError::ValidationError {
        field: "test error".into(),
    });

    result.map_err(|err| {
        error::ErrorBadRequest(matches!(err, MyError::ValidationError { .. }))
    })
}
