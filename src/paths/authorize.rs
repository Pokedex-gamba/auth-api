use std::collections::HashSet;

use actix_web::{
    http::{header::AUTHORIZATION, StatusCode},
    post,
    web::Data,
    HttpResponse, Responder,
};
use serde::Deserialize;
use sqlx::{query_as, types::Json, PgPool};

use crate::{
    jwt_stuff::{TokenUtils, UserId},
    models::token::Token,
    util::response_from_error,
};

#[derive(Deserialize)]
struct Row {
    all_grants: Json<HashSet<String>>,
}

#[post("/authorize")]
pub async fn authorize(
    user_id: Option<UserId>,
    token_utils: Data<TokenUtils>,
    pool: Data<PgPool>,
) -> impl Responder {
    let res = match &user_id {
        Some(user_id) => {
            query_as!(
                Row,
                r#"select all_grants as "all_grants!: Json<HashSet<String>>" from grants_with_subgrants where id = (select "grant" from users where id = $1)"#,
                **user_id,
            )
            .fetch_one(pool.get_ref())
            .await
        },
        None => {
            query_as!(
                Row,
                r#"select all_grants as "all_grants!: Json<HashSet<String>>" from grants_with_subgrants where id = (select id from grants where name = 'role::public')"#,
            )
            .fetch_one(pool.get_ref())
            .await
        },
    };
    handle_response(user_id, res, &token_utils)
}

fn handle_response(
    user_id: Option<UserId>,
    res: Result<Row, sqlx::Error>,
    token_utils: &TokenUtils,
) -> impl Responder {
    match res {
        Ok(row) => {
            let token = token_utils.encode_grants(user_id.map(|uid| *uid), row.all_grants.0);

            HttpResponse::Ok()
                .insert_header((AUTHORIZATION, format!("Bearer {token}")))
                .json(Token { token })
        }
        Err(sqlx::Error::RowNotFound) => match user_id {
            Some(user_id) => {
                tracing::warn!(
                    "Token with 'user_id: {}' exists, but user no longer exists",
                    *user_id
                );
                response_from_error(format!("User doesn't exist"), StatusCode::NOT_FOUND)
            }
            None => {
                tracing::warn!("Grant 'role::public' doesn't exists -> guest user doesn't have any permissions");
                let token = token_utils.encode_grants(None, HashSet::new());

                HttpResponse::Ok()
                    .insert_header((AUTHORIZATION, format!("Bearer {token}")))
                    .json(Token { token })
            }
        },
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
