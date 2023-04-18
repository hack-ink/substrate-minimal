//! Substrate metadata's minimal implementation.

pub use Metadata as MetadataMinimal;

// crates.io
use fxhash::FxHashMap;
use scale_info::form::PortableForm;
use substorager::StorageHasher;
// substrate-minimal
use crate::LatestRuntimeMetadata;

/// Some useful functions to access the metadata.
pub trait Meta {
	/// Get the storage entry.
	fn storage<'a, 'b>(&'a self, pallet: &str, item: &'b str) -> Option<StorageEntry<'b>>
	where
		'a: 'b;
}

trait KV {
	type V;

	fn kv(self) -> (String, Self::V);
}

/// Metadata minimal implementation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Metadata {
	/// Pallet metadata(s).
	pub pallets: FxHashMap<String, PalletMetadata>,
}
impl From<LatestRuntimeMetadata> for Metadata {
	fn from(v: LatestRuntimeMetadata) -> Self {
		Self { pallets: v.pallets.into_iter().map(KV::kv).collect() }
	}
}
impl Meta for Metadata {
	fn storage<'a, 'b>(&'a self, pallet: &str, item: &'b str) -> Option<StorageEntry<'b>>
	where
		'a: 'b,
	{
		self.pallets.get(pallet).and_then(|p| p.storages.as_ref()).and_then(|s| {
			s.entries.get(item).map(|e| StorageEntry { prefix: &s.prefix, item, r#type: &e.r#type })
		})
	}
}

/// Storage entry minimal implementation.
pub struct StorageEntry<'a> {
	/// Pallet prefix.
	pub prefix: &'a str,
	/// Item name.
	pub item: &'a str,
	/// Storage type.
	pub r#type: &'a StorageEntryType,
}

/// Pallet metadata minimal implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PalletMetadata {
	/// Pallet index.
	pub index: u8,
	/// Pallet storage metadata.
	pub storages: Option<PalletStorageMetadata>,
	// pub calls: Option<CallMetadata>,
	// pub events: Option<EventMetadata>,
	// pub constants: Vec<ConstantMetadata>,
	// pub error: Option<ErrorMetadata>,
}
impl KV for frame_metadata::PalletMetadata<PortableForm> {
	type V = PalletMetadata;

	fn kv(self) -> (String, Self::V) {
		(self.name, PalletMetadata { index: self.index, storages: self.storage.map(Into::into) })
	}
}

/// Pallet storage minimal implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PalletStorageMetadata {
	/// Pallet prefix.
	pub prefix: String,
	/// Pallet storage entries.
	pub entries: FxHashMap<String, StorageEntryMetadata>,
}
impl From<frame_metadata::PalletStorageMetadata<PortableForm>> for PalletStorageMetadata {
	fn from(v: frame_metadata::PalletStorageMetadata<PortableForm>) -> Self {
		Self { prefix: v.prefix, entries: v.entries.into_iter().map(KV::kv).collect() }
	}
}

/// Pallet storage entry minimal implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageEntryMetadata {
	/// Storage entry type.
	pub r#type: StorageEntryType,
}
impl KV for frame_metadata::StorageEntryMetadata<PortableForm> {
	type V = StorageEntryMetadata;

	fn kv(self) -> (String, Self::V) {
		(self.name, StorageEntryMetadata { r#type: self.ty.into() })
	}
}

/// Storage entry type minimal implementation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StorageEntryType {
	/// Plain storage.
	Plain,
	/// Map storage.
	Map(Vec<StorageHasher>),
}
impl From<frame_metadata::StorageEntryType<PortableForm>> for StorageEntryType {
	fn from(v: frame_metadata::StorageEntryType<PortableForm>) -> Self {
		match v {
			frame_metadata::StorageEntryType::Plain(_) => Self::Plain,
			frame_metadata::StorageEntryType::Map { hashers, .. } => Self::Map(
				hashers
					.into_iter()
					.map(|h| match h {
						frame_metadata::StorageHasher::Blake2_128 => StorageHasher::Blake2_128,
						frame_metadata::StorageHasher::Blake2_256 => StorageHasher::Blake2_256,
						frame_metadata::StorageHasher::Blake2_128Concat =>
							StorageHasher::Blake2_128Concat,
						frame_metadata::StorageHasher::Twox128 => StorageHasher::Twox128,
						frame_metadata::StorageHasher::Twox256 => StorageHasher::Twox256,
						frame_metadata::StorageHasher::Twox64Concat => StorageHasher::Twox64Concat,
						frame_metadata::StorageHasher::Identity => StorageHasher::Identity,
					})
					.collect(),
			),
		}
	}
}
