use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{post, Responder};
use garde_actix_web::web::Json;
use sqlx::{query, PgPool};

use crate::hash_utils::{make_hash, make_salt};
use crate::jwt_stuff::TokenUtils;
use crate::macros::resp_200_Ok_json;
use crate::models::register_data::RegisterData;
use crate::models::token::Token;
use crate::util::response_from_error;

#[actix_web_grants::protect("svc::auth_api::route::/auth/register")]
#[post("/register")]
pub async fn register(
    data: Json<RegisterData>,
    token_utils: Data<TokenUtils>,
    pool: Data<PgPool>,
) -> impl Responder {
    let salt = make_salt();
    let hash = make_hash(&data.password, &salt);

    match query!(
        "insert into users (email, hash, salt) values ($1, $2, $3) returning users.id",
        data.email.to_lowercase(),
        hash.to_vec(),
        salt
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(row) => {
            let token = Token {
                token: token_utils.encode_public_token(row.id),
            };

            resp_200_Ok_json!(token)
        }
        Err(sqlx::Error::Database(error)) => {
            if error.is_unique_violation() {
                response_from_error(
                    "Register request violates unique constraints",
                    StatusCode::BAD_REQUEST,
                )
            } else {
                response_from_error(
                    format!("unhandled error - {}", error),
                    StatusCode::BAD_REQUEST,
                )
            }
        }
        Err(error) => response_from_error(
            format!("unhandled error - {}", error),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
