use std::fmt::{Debug, Display};

use actix_web::{
    body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, HttpResponseBuilder, ResponseError,
};
use serde::Serialize;

use crate::RESPONSE_HEADER;

#[derive(Debug, Serialize)]
pub struct JsonError<Err> {
    error: Err,
    #[serde(skip)]
    status_code: StatusCode,
    #[serde(skip)]
    inject_into_header: bool,
}

impl<Err> JsonError<Err> {
    pub fn new(error: Err, status_code: StatusCode) -> Self {
        Self {
            error,
            status_code,
            inject_into_header: false,
        }
    }

    pub fn inject_into_header(mut self) -> Self {
        self.inject_into_header = true;
        self
    }
}

impl<Err: Debug> Display for JsonError<Err> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonError")
            .field("error", &self.error)
            .field("status_code", &self.status_code)
            .finish()
    }
}

impl From<&dyn actix_web::ResponseError> for JsonError<String> {
    fn from(value: &dyn actix_web::ResponseError) -> Self {
        Self {
            status_code: value.status_code(),
            error: value.to_string(),
            inject_into_header: false,
        }
    }
}

impl<Err: Serialize + Debug> ResponseError for JsonError<Err> {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponseBuilder::new(self.status_code)
            .insert_header((RESPONSE_HEADER, body.as_str()))
            .body(body)
    }
}

pub fn config_json_error_handler<Err: actix_web::ResponseError + 'static>(
    err: Err,
    _: &HttpRequest,
) -> actix_web::Error {
    JsonError::from(&err as &dyn actix_web::ResponseError).into()
}
