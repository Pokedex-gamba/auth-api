use std::fmt::Debug;

use actix_web::{http::StatusCode, Error, HttpResponse};
use serde::Serialize;

use crate::{empty_error::EmptyError, json_error::JsonError, IS_DEBUG_ON};

pub fn response_from_error(
    error: impl Serialize + Debug + 'static,
    status_code: StatusCode,
) -> HttpResponse {
    get_actix_error(error, status_code).error_response()
}

pub fn get_actix_error(error: impl Serialize + Debug + 'static, status_code: StatusCode) -> Error {
    if unsafe { IS_DEBUG_ON } {
        JsonError::new(error, status_code).into()
    } else {
        EmptyError::new(status_code).into()
    }
}
