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
            response_from_error(format!("User doesn't exist"), StatusCode::NOT_FOUND)
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
