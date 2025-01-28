use argon2::Params;

/// The Argon2d parameters used for hashing.
pub fn get_mining_params() -> Params {
    Params::new(
        32 * 1024,  // m_cost: memory usage in KiB Example: (32,768 = 32 MB).
        1,          // t_cost: number of iterations
        1,          // p_cost: number of parallelism (lanes)
        Some(32),   // output length in bytes
    )
    .expect("Failed to create Argon2 parameters")
}
