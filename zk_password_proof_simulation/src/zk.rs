//here we hash each input, from the secret password to the user attempts

use sha2::{Digest, Sha256};

pub fn hash_password(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}