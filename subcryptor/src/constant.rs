//! Subcryptor constant collections.

/// PKCS8 header.
pub const PKCS8_HEADER: [u8; 16] = [48, 83, 2, 1, 1, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32];
/// Seed offset.
pub const SEED_OFFSET: usize = PKCS8_HEADER.len();
/// Secret key length.
pub const SECRET_KEY_LEN: usize = 64;
