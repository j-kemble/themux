// Socket authentication.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Verify a relay token against a stored SHA-256 hash.
pub fn verify_relay_token(token: &str, expected_hash: &str) -> bool {
    let mut mac = HmacSha256::new_from_slice(token.as_bytes()).unwrap();
    let computed = hex::encode(mac.finalize().into_bytes());
    computed == expected_hash
}

/// Generate a secure random relay token.
pub fn generate_relay_token() -> String {
    use rand::Rng;
    let bytes: [u8; 32] = rand::rng().random();
    hex::encode(bytes)
}
