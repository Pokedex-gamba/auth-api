use actix_web::web::ServiceConfig;
pub mod authorize;
pub mod login;
pub mod register;

pub fn configure_grants_jwt(cfg: &mut ServiceConfig) {
    cfg.service(login::login).service(register::register);
}

pub fn configure_public_token_jwt(cfg: &mut ServiceConfig) {
    cfg.service(authorize::authorize);
}
