// crates.io
use base64::{engine::general_purpose::STANDARD, Engine};
// subcryptor
use crate::prelude::*;

/// Decode the base64 encoded data.
pub fn base64_decode<T>(data: T) -> Result<Vec<u8>>
where
	T: AsRef<[u8]>,
{
	Ok(STANDARD.decode(data)?)
}
