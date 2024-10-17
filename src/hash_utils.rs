pub fn make_salt() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const SALT_LEN: usize = 128;
    let mut rng = rand::thread_rng();

    let salt: String = (0..SALT_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    salt
}

pub fn make_hash(password: &str, salt: &str) -> [u8; 32] {
    argon2rs::argon2i_simple(password, salt)
}

pub fn verify_password(hash: &[u8; 32], salt: &str, password: &str) -> bool {
    make_hash(password, salt) == *hash
}
