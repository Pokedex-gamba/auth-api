use std::sync::Arc;

use actix_jwt_middleware::JwtMiddleware;
use actix_web::{
    http::StatusCode,
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::{self, Data},
    App, HttpMessage, HttpServer,
};
use actix_web_grants::{GrantErrorConfig, GrantsConfig};
use docs::AutoTagAddon;
use util::{get_actix_error, get_config_error_handler};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use utoipauto::utoipauto;

mod docs;
mod empty_error;
mod hash_utils;
mod json_error;
mod jwt_stuff;
mod macros;
mod models;
mod paths;
mod util;

static mut IS_DEBUG_ON: bool = false;

async fn default_handler_debug(req: actix_web::HttpRequest) -> impl actix_web::Responder {
    actix_web::HttpResponse::NotFound().body(format!("{:#?}", req))
}
async fn default_handler() -> impl actix_web::Responder {
    actix_web::HttpResponse::NotFound().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    #[utoipauto]
    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Pokemon API"
        ),
        modifiers(&AutoTagAddon)
    )]
    struct ApiDoc;

    let is_debug_on = std::env::var("DEBUG")
        .map(|val| val == "1")
        .unwrap_or_default();
    unsafe {
        IS_DEBUG_ON = is_debug_on;
    }
    tracing::info!(
        "Debug is {}",
        if is_debug_on { "enabled" } else { "disabled" }
    );

    let bind_address = std::env::var("ADDRESS").unwrap_or("0.0.0.0:80".into());

    let jwt_stuff::Keys {
        grants_token_keys:
            jwt_stuff::KeyPair {
                decoding_key: grants_decoding_key,
                encoding_key: grants_encoding_key,
            },
        public_token_keys:
            jwt_stuff::KeyPair {
                decoding_key: public_token_decoding_key,
                encoding_key: public_token_encoding_key,
            },
    } = jwt_stuff::get_keys();

    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        tracing::error!("Database url environmental variable is not set",);
        tracing::info!("Fatal error encountered halting!");
        std::thread::park();
        panic!();
    };
    let pool = Data::new(
        sqlx::postgres::PgPoolOptions::new()
            .connect(&database_url)
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Error encounterder when connecting to database: {e}");
                panic!();
            }),
    );

    let token_utils = Data::new(jwt_stuff::TokenUtils::new(
        grants_encoding_key,
        public_token_encoding_key,
    ));

    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_required_spec_claims(&["exp", "nbf"]);

    let actix_json_config =
        actix_web::web::JsonConfig::default().error_handler(get_config_error_handler());

    let grade_json_config =
        garde_actix_web::web::JsonConfig::default().error_handler(get_config_error_handler());

    let jwt_error_handler = move |error: actix_jwt_middleware::JwtDecodeErrors| {
        get_actix_error(error.to_error_string(), StatusCode::BAD_REQUEST)
    };

    let jwt_public_token_middleware = JwtMiddleware::<jwt_stuff::PublicTokenData>::new(
        public_token_decoding_key,
        validation.clone(),
    )
    .error_handler(jwt_error_handler);

    let grants_string_error_config = GrantErrorConfig::<String>::default()
            .error_handler(move |condition, grants| {
                let msg = format!(
                    "Insufficient permissions. Condition '{}' needs to be fulfilled. Grants provided: {:?}",
                    condition, grants
                );
                get_actix_error(msg, StatusCode::FORBIDDEN).error_response()
            });

    let grants_config = GrantsConfig::default().missing_auth_details_error_handler(move || {
        get_actix_error("Authorization header is missing", StatusCode::UNAUTHORIZED)
    });

    let jwt_grants_middleware =
        JwtMiddleware::<jwt_stuff::GrantsTokenData>::new(grants_decoding_key, validation)
            .error_handler(jwt_error_handler)
            .success_handler(|req, jwt_stuff::GrantsTokenData { grants, user_id }| {
                req.extensions_mut()
                    .insert(actix_web_grants::authorities::AuthDetails {
                        authorities: Arc::new(grants),
                    });
                req.extensions_mut().insert(jwt_stuff::UserId::new(user_id));
            });

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(grants_string_error_config.clone())
            .app_data(grants_config.clone())
            .app_data(actix_json_config.clone())
            .app_data(grade_json_config.clone())
            .app_data(token_utils.clone())
            .app_data(pool.clone());
        if is_debug_on {
            app = app.service(Scalar::with_url("/docs", ApiDoc::openapi()));
        }
        app.service(
            web::scope("")
                .wrap(jwt_grants_middleware.clone())
                .configure(paths::configure_grants_jwt),
        )
        .service(
            web::scope("")
                .wrap(jwt_public_token_middleware.clone())
                .configure(paths::configure_public_token_jwt),
        )
        .default_service(if is_debug_on {
            web::to(default_handler_debug)
        } else {
            web::to(default_handler)
        })
    })
    .bind(bind_address)
    .expect("Failed to bind server to address")
    .run()
    .await
}
