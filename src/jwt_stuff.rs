use std::{collections::HashSet, future::Future, pin::Pin};

use actix_web::{http::StatusCode, Error, FromRequest, HttpMessage};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::util::get_actix_error;

#[derive(Serialize)]
pub struct GrantsTokenData {
    pub user_id: Uuid,
    pub grants: HashSet<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PublicTokenData {
    pub user_id: Uuid,
}

impl FromRequest for PublicTokenData {
    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let data = req
            .extensions()
            .get::<PublicTokenData>()
            .cloned()
            .ok_or_else(|| {
                get_actix_error("Authorization header is missing", StatusCode::UNAUTHORIZED)
            });

        Box::pin(async move { data })
    }
}

pub struct KeyPair {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

pub struct Keys {
    pub grants_token_keys: KeyPair,
    pub public_token_keys: KeyPair,
}

pub fn get_keys() -> Keys {
    macro_rules! get_key {
        ($env:expr, $name:expr, $fs_loc:expr, $key_ident:ident) => {{
            let key_str = match std::env::var($env) {
                Ok(key) => {
                    tracing::info!("Using {} key from environment", $name);
                    key
                }
                Err(_) => std::fs::read_to_string($fs_loc)
                    .inspect(|_| {
                        tracing::info!("Using {} key from filesystem", $name);
                    })
                    .unwrap_or_else(|_| {
                        tracing::error!(
                            "Couldn't find {} key in either environment nor filesystem",
                            $name
                        );
                        tracing::info!("Fatal error encountered halting!");
                        std::thread::park();
                        panic!();
                    }),
            };
            let jwt_key = match $key_ident::from_rsa_pem(key_str.as_bytes()) {
                Ok(key) => key,
                Err(e) => {
                    tracing::error!("Parsing of {} key failed with error: {}", $name, e);
                    tracing::info!("Fatal error encountered halting!");
                    std::thread::park();
                    panic!();
                }
            };
            jwt_key
        }};
    }

    let grants_decoding_key = get_key!(
        "GRANTS_DECODING_KEY",
        "grants decoding",
        "./grants_decoding_key",
        DecodingKey
    );

    let grants_encoding_key = get_key!(
        "GRANTS_ENCODING_KEY",
        "grants encoding",
        "./grants_encoding_key",
        EncodingKey
    );

    let public_token_decoding_key = get_key!(
        "TOKEN_DECODING_KEY",
        "public token decoding",
        "./token_decoding_key",
        DecodingKey
    );
    let public_token_encoding_key = get_key!(
        "TOKEN_ENCODING_KEY",
        "public token encoding",
        "./token_encoding_key",
        EncodingKey
    );

    Keys {
        grants_token_keys: KeyPair {
            encoding_key: grants_encoding_key,
            decoding_key: grants_decoding_key,
        },
        public_token_keys: KeyPair {
            decoding_key: public_token_decoding_key,
            encoding_key: public_token_encoding_key,
        },
    }
}

pub struct TokenUtils {
    grants_encoding_key: EncodingKey,
    public_token_encoding_key: EncodingKey,
}

impl TokenUtils {
    pub fn new(grants_encoding_key: EncodingKey, public_token_encoding_key: EncodingKey) -> Self {
        Self {
            grants_encoding_key,
            public_token_encoding_key,
        }
    }
}
