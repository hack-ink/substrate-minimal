//! Minimal implementation of Substrate metadata.

#![deny(missing_docs)]
#![deny(unused_crate_dependencies)]

pub mod error;
pub use error::Error;

pub mod metadata;
pub use metadata::*;

#[cfg(feature = "cmp")] pub mod cmp;

pub use frame_metadata::{self, RuntimeMetadataV14 as LatestRuntimeMetadata};
pub use parity_scale_codec;
pub use scale_info;

// crates.io
use parity_scale_codec::Decode;

/// Main result.
pub type Result<T> = std::result::Result<T, Error>;

/// Try extracting [`LatestRuntimeMetadata`] from [`frame_metadata::RuntimeMetadataPrefixed`].
pub fn unprefix_metadata(
	metadata: frame_metadata::RuntimeMetadataPrefixed,
) -> Result<LatestRuntimeMetadata> {
	match metadata.1 {
		frame_metadata::RuntimeMetadata::V14(metadata) => Ok(metadata),
		metadata => Err(Error::UnsupportedVersion(metadata.version())),
	}
}
/// Try extracting [`LatestRuntimeMetadata`] from [`AsRef<str>`].
pub fn unprefix_raw_metadata<R>(raw_metadata: R) -> Result<LatestRuntimeMetadata>
where
	R: AsRef<str>,
{
	unprefix_metadata(
		frame_metadata::RuntimeMetadataPrefixed::decode(
			&mut &*array_bytes::hex2bytes(raw_metadata.as_ref())
				.map_err(error::Error::ArrayBytes)?,
		)
		.map_err(error::Error::Codec)?,
	)
}

/// Try extracting [`Metadata`] from [`frame_metadata::RuntimeMetadataPrefixed`].
pub fn unprefix_metadata_minimal(
	metadata: frame_metadata::RuntimeMetadataPrefixed,
) -> Result<Metadata> {
	Ok(unprefix_metadata(metadata)?.into())
}
/// Try extracting [`Metadata`] from [`AsRef<str>`].
pub fn unprefix_raw_metadata_minimal<R>(raw_metadata: R) -> Result<Metadata>
where
	R: AsRef<str>,
{
	Ok(unprefix_raw_metadata(raw_metadata)?.into())
}
