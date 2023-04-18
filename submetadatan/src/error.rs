//! Submetadatan error collections.

// crates.io
use thiserror::Error as ThisError;

/// Main error.
#[allow(missing_docs)]
#[derive(Debug, ThisError)]
pub enum Error {
	#[error("unsupported version, {0:?}")]
	UnsupportedVersion(u32),
	#[error("{0:?}")]
	ArrayBytes(array_bytes::Error),
	#[error(transparent)]
	Codec(parity_scale_codec::Error),
}
