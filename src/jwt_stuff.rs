use std::collections::HashSet;

use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct GrantsData {
    pub user_id: Uuid,
    pub grants: HashSet<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PublicTokenData {
    pub user_id: Uuid,
}

pub struct KeyPair {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

pub struct Keys {
    pub grants_encoding_key: EncodingKey,
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
        grants_encoding_key,
        public_token_keys: KeyPair {
            decoding_key: public_token_decoding_key,
            encoding_key: public_token_encoding_key,
        },
    }
}

pub struct TokenUtils {
    pub keys: Keys,
    pub validation: Validation,
}

impl TokenUtils {
    pub fn init() -> Self {
        let keys = get_keys();

        let mut validation = Validation::new(jsonwebtoken::Algorithm::RS256);
        validation.set_required_spec_claims(&["exp", "nbf"]);

        Self { keys, validation }
    }
}
