//! Subcryptor error collections.

// crates.io
use thiserror::Error as ThisError;

/// Main error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Error {
	#[error("[subcryptor] {0:?}")]
	ArrayBytes(array_bytes::Error),
	#[error(transparent)]
	Base64Decode(#[from] base64::DecodeError),
	#[error(transparent)]
	CryptoSecretBox(#[from] CryptoSecretBox),
	#[error("[subcryptor] from base58 error, {0:?}")]
	FromBase58(base58::FromBase58Error),
	#[error("[subcryptor] invalid prefix, {0:?}")]
	InvalidPrefix(u8),
	#[error("[subcryptor] invalid ss58 address, {0:?}")]
	InvalidSs58Address(String),
	#[error(transparent)]
	Scrypt(#[from] scrypt::errors::InvalidOutputLen),
	#[error("[subcryptor] unsupported encryption type")]
	UnsupportedEncryptionType,
	#[error("[subcryptor] unsupported network, {0:?}")]
	UnsupportedNetwork(String),
}

/// Crypto secretbox error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum CryptoSecretBox {
	#[error("[subcryptor] crypto secretbox general error, {0:?}")]
	General(crypto_secretbox::Error),
	#[error("[subcryptor] crypto secretbox cipher error, {0:?}")]
	Cipher(crypto_secretbox::cipher::InvalidLength),
}
