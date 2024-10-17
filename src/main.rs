use actix_jwt_middleware::JwtMiddleware;
use actix_web::{
    http::StatusCode,
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::{self, Data, JsonConfig},
    App, HttpServer,
};
use docs::AutoTagAddon;
use empty_error::EmptyError;
use json_error::JsonError;
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
        grants_encoding_key,
        public_token_keys:
            jwt_stuff::KeyPair {
                decoding_key: public_token_decoding_key,
                encoding_key: public_token_encoding_key,
            },
    } = jwt_stuff::get_keys();

    let token_utils = Data::new(jwt_stuff::TokenUtils::new(
        grants_encoding_key,
        public_token_encoding_key,
    ));

    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
    validation.set_required_spec_claims(&["exp", "nbf"]);

    let json_config = JsonConfig::default().error_handler(if is_debug_on {
        json_error::config_json_error_handler
    } else {
        empty_error::config_empty_error_handler
    });

    let jwt_error_handler = move |error: actix_jwt_middleware::JwtDecodeErrors| {
        let code = StatusCode::BAD_REQUEST;
        if is_debug_on {
            JsonError::new(error.to_error_string(), code).into()
        } else {
            EmptyError::new(code).into()
        }
    };

    let jwt_public_token_middleware =
        JwtMiddleware::<jwt_stuff::PublicTokenData>::new(public_token_decoding_key, validation)
            .error_handler(jwt_error_handler);

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(json_config.clone())
            .app_data(token_utils.clone());
        if is_debug_on {
            app = app.service(Scalar::with_url("/docs", ApiDoc::openapi()));
        }
        app.configure(paths::configure_public)
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
