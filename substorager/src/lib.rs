//! Minimal implementation of Substrate storage.

#![deny(missing_docs)]
#![deny(unused_crate_dependencies)]

#[cfg(test)] mod test;

// std
use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	ops::Deref,
};
// crates.io
#[cfg(feature = "codec")] use parity_scale_codec::{Decode, Encode};

/// Storage key.
///
/// Substrate reference(s):
/// - <https://github.com/paritytech/substrate/blob/c4d36065764ee23aeb3ccd181c4b6ecea8d2447a/primitives/storage/src/lib.rs#L35-L43>
#[derive(Debug, Default)]
pub struct StorageKey(pub Vec<u8>);
impl StorageKey {
	/// Create an empty [`StorageKey`].
	pub fn new() -> Self {
		Default::default()
	}
}
impl AsRef<[u8]> for StorageKey {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}
impl Deref for StorageKey {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl Display for StorageKey {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", array_bytes::bytes2hex("0x", &self.0))
	}
}
impl From<Vec<u8>> for StorageKey {
	fn from(v: Vec<u8>) -> Self {
		Self(v)
	}
}
impl<const N: usize> From<[u8; N]> for StorageKey {
	fn from(a: [u8; N]) -> Self {
		Self(a.to_vec())
	}
}
impl From<&[u8]> for StorageKey {
	fn from(a: &[u8]) -> Self {
		Self(a.to_vec())
	}
}

/// Storage hasher.
///
/// Substrate reference(s):
/// - <https://github.com/paritytech/substrate/blob/c4d36065764ee23aeb3ccd181c4b6ecea8d2447a/frame/support/src/hash.rs#L25-L34>
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "codec", derive(Encode, Decode))]
pub enum StorageHasher {
	#[allow(missing_docs)]
	Blake2_128,
	#[allow(missing_docs)]
	Blake2_256,
	#[allow(missing_docs)]
	Blake2_128Concat,
	#[allow(missing_docs)]
	Twox128,
	#[allow(missing_docs)]
	Twox256,
	#[allow(missing_docs)]
	Twox64Concat,
	#[allow(missing_docs)]
	Identity,
}
impl StorageHasher {
	/// Hash the data and make it into a [`StorageKey`].
	pub fn hash<A>(&self, data: A) -> StorageKey
	where
		A: AsRef<[u8]>,
	{
		match self {
			Self::Blake2_128 => subhasher::blake2_128(data).into(),
			Self::Blake2_256 => subhasher::blake2_256(data).into(),
			Self::Blake2_128Concat => subhasher::blake2_128_concat(data).into(),
			Self::Twox128 => subhasher::twox128(data).into(),
			Self::Twox256 => subhasher::twox256(data).into(),
			Self::Twox64Concat => subhasher::twox64_concat(data).into(),
			Self::Identity => subhasher::identity(data.as_ref()).into(),
		}
	}
}
impl AsRef<StorageHasher> for StorageHasher {
	fn as_ref(&self) -> &Self {
		self
	}
}

/// Calculate the storage key of a pallet `StorageValue` item.
pub fn storage_value_key<A>(pallet: A, item: A) -> StorageKey
where
	A: AsRef<[u8]>,
{
	let mut k = Vec::new();

	k.extend_from_slice(&subhasher::twox128(pallet));
	k.extend_from_slice(&subhasher::twox128(item));

	k.into()
}

/// Calculate the storage key of a pallet `StorageNMap` item.
pub fn storage_n_map_key<A, Aa, Aa1, Aa2>(pallet: A, item: A, keys: Aa) -> StorageKey
where
	A: AsRef<[u8]>,
	Aa: AsRef<[(Aa1, Aa2)]>,
	Aa1: AsRef<StorageHasher>,
	Aa2: AsRef<[u8]>,
{
	let mut k = storage_value_key(pallet, item);

	keys.as_ref().iter().for_each(|(h, d)| k.0.extend_from_slice(&h.as_ref().hash(d)));

	k
}
