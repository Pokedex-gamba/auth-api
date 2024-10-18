use actix_web::{
    http::StatusCode,
    post,
    web::{Data, Json},
    Responder,
};
use sqlx::{query, PgPool};

use crate::{
    hash_utils::verify_password,
    jwt_stuff::TokenUtils,
    macros::resp_200_Ok_json,
    models::{login_data::LoginData, token::Token},
    util::response_from_error,
};

#[utoipa::path(
    context_path = "/auth",
    request_body = LoginData,
    responses(
        (status = 200, description = "Returns valid public token", body = Token),
        (status = 400, description = "Wrong password for user<br>or<br>Some unhandled db error"),
        (status = 404, description = "User doesn't exists"),
        (status = 500, description = "Failed to fetch/parse data from db"),
    ),
    security(
        ("jwt_grants" = ["svc::auth_api::route::/auth/login"]),
    )
)]
#[actix_web_grants::protect("svc::auth_api::route::/auth/login")]
#[post("/login")]
pub async fn login(
    data: Json<LoginData>,
    token_utils: Data<TokenUtils>,
    pool: Data<PgPool>,
) -> impl Responder {
    match query!(
        "select id, hash, salt from users where email = $1",
        data.email.to_lowercase(),
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(row) => {
            if verify_password(
                row.hash.as_slice().try_into().unwrap(),
                &row.salt,
                &data.password,
            ) {
                let token = Token {
                    token: token_utils.encode_public_token(row.id),
                };

                resp_200_Ok_json!(token)
            } else {
                response_from_error("Wrong credentials", StatusCode::BAD_REQUEST)
            }
        }
        Err(sqlx::Error::RowNotFound) => {
            response_from_error("User doesn't exist".to_string(), StatusCode::NOT_FOUND)
        }
        Err(sqlx::Error::Database(error)) => response_from_error(
            format!("unhandled error - {}", error),
            StatusCode::BAD_REQUEST,
        ),
        Err(error) => response_from_error(
            format!("unhandled error - {}", error),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
