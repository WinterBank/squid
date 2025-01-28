use argon2::{Argon2, Algorithm, Version, Params};
use crate::mining::mining_params::get_mining_params;

/// Validate a mining submission against the difficulty target.
pub fn validate_hash(hash: &str, salt: &str, difficulty: u64) -> bool {
    // Convert the hash and salt to bytes.
    let hash_bytes = hex::decode(hash).unwrap_or_else(|_| panic!("Invalid hash format"));
    let salt_bytes = hex::decode(salt).unwrap_or_else(|_| panic!("Invalid salt format"));

    // Count the leading zero bits in the hash.
    let leading_zero_bits = count_leading_zero_bits(&hash_bytes);

    // Check if the hash meets the difficulty target.
    leading_zero_bits >= difficulty
}

/// Count the number of leading zero bits in a byte array.
fn count_leading_zero_bits(bytes: &[u8]) -> u64 {
    let mut count = 0;
    for byte in bytes {
        if *byte == 0 {
            count += 8;
        } else {
            count += byte.leading_zeros() as u64;
            break;
        }
    }
    count
}

/// Generate an Argon2d hash for a given input and salt.
pub fn generate_argon2d_hash(input: &str, salt: &str) -> Vec<u8> {
    let params = get_mining_params();
    let argon2 = Argon2::new(Algorithm::Argon2d, Version::V0x13, params);
    let mut hash = vec![0u8; 32];
    argon2
        .hash_password_into(input.as_bytes(), salt.as_bytes(), &mut hash)
        .expect("Argon2 hashing failed");
    hash
}