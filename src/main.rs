use actix_web::{
    middleware::{Compress, Logger, NormalizePath, TrailingSlash},
    web::{self, Data, JsonConfig},
    App, HttpServer,
};
use docs::AutoTagAddon;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use utoipauto::utoipauto;

mod docs;
mod empty_error;
mod json_error;
mod macros;
mod paths;

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
    tracing::info!(
        "Debug is {}",
        if is_debug_on { "enabled" } else { "disabled" }
    );

    let bind_address = std::env::var("ADDRESS").unwrap_or("0.0.0.0:80".into());

    HttpServer::new(move || {
        let json_config = JsonConfig::default().error_handler(if is_debug_on {
            json_error::config_json_error_handler
        } else {
            empty_error::config_empty_error_handler
        });

        let mut app = App::new()
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(json_config)
        if is_debug_on {
            app = app.service(Scalar::with_url("/docs", ApiDoc::openapi()));
        }
        app.configure(paths::configure)
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
