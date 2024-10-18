use std::fmt::Debug;

use actix_web::{http::StatusCode, Error, HttpRequest, HttpResponse};
use serde::Serialize;

use crate::{
    empty_error::{self, EmptyError},
    json_error::{self, JsonError},
    IS_DEBUG_ON,
};

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

pub fn get_config_error_handler<E>() -> impl Fn(E, &HttpRequest) -> Error
where
    E: actix_web::ResponseError + 'static,
{
    if unsafe { IS_DEBUG_ON } {
        json_error::config_json_error_handler
    } else {
        empty_error::config_empty_error_handler
    }
}
