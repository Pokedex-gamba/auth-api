use std::fmt::Debug;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

use crate::{empty_error::EmptyError, json_error::JsonError, IS_DEBUG_ON};

pub fn response_from_error(error: impl Serialize + Debug, status_code: StatusCode) -> HttpResponse {
    if unsafe { IS_DEBUG_ON } {
        JsonError::new(error, status_code).error_response()
    } else {
        EmptyError::new(status_code).error_response()
    }
}
