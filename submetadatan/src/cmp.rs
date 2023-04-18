//! Utilities for comparing the metadata of two Substrate runtimes.

// std
use std::any::TypeId;
// crates.io
use scale_info::{
	form::PortableForm, interner::UntrackedSymbol, Field, Type, TypeDef, TypeParameter, Variant, *,
};

/// Compare two [`frame_metadata::StorageEntryMetadata`] and return the [`bool`] result.
pub fn storage_entry(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &frame_metadata::StorageEntryMetadata<PortableForm>,
	b: &frame_metadata::StorageEntryMetadata<PortableForm>,
) -> bool {
	a.name == b.name
		&& a.modifier == b.modifier
		&& a.default == b.default
		&& a.docs == b.docs
		&& storage_entry_type(a_types, b_types, &a.ty, &b.ty)
}

/// Compare two [`frame_metadata::StorageEntryType`] and return the [`bool`] result.
pub fn storage_entry_type(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &frame_metadata::StorageEntryType<PortableForm>,
	b: &frame_metadata::StorageEntryType<PortableForm>,
) -> bool {
	match a {
		frame_metadata::StorageEntryType::Plain(a) => match b {
			frame_metadata::StorageEntryType::Plain(b) => untracked_symbol(a_types, b_types, a, b),
			_ => false,
		},
		frame_metadata::StorageEntryType::Map {
			hashers: a_hashers,
			key: a_key,
			value: a_value,
		} => match b {
			frame_metadata::StorageEntryType::Map {
				hashers: b_hashers,
				key: b_key,
				value: b_value,
			} =>
				a_hashers == b_hashers
					&& untracked_symbol(a_types, b_types, a_key, b_key)
					&& untracked_symbol(a_types, b_types, a_value, b_value),
			_ => false,
		},
	}
}

/// Compare two [`UntrackedSymbol`] and return the [`bool`] result.
pub fn untracked_symbol(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &UntrackedSymbol<TypeId>,
	b: &UntrackedSymbol<TypeId>,
) -> bool {
	r#type(a_types, b_types, a_types.resolve(a.id), b_types.resolve(b.id))
}

/// Compare two [`Type`] and return the [`bool`] result.
pub fn r#type(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: Option<&Type<PortableForm>>,
	b: Option<&Type<PortableForm>>,
) -> bool {
	if let Some(a) = a {
		if let Some(b) = b {
			a.path == b.path
				&& a.docs == b.docs
				&& type_params(a_types, b_types, &a.type_params, &b.type_params)
				&& type_def(a_types, b_types, &a.type_def, &b.type_def)
		} else {
			false
		}
	} else {
		b.is_none()
	}
}

/// Compare two [`TypeParameter`] and return the [`bool`] result.
pub fn type_params(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &[TypeParameter<PortableForm>],
	b: &[TypeParameter<PortableForm>],
) -> bool {
	if a.is_empty() && b.is_empty() {
		return true;
	}
	if a.len() != b.len() {
		return false;
	}

	for (a, b) in a.iter().zip(b.iter()) {
		if a.name != b.name {
			return false;
		}

		if let Some(a_type) = &a.ty {
			if let Some(b_type) = &b.ty {
				return untracked_symbol(a_types, b_types, a_type, b_type);
			} else {
				return false;
			}
		} else if b.ty.is_some() {
			return false;
		}
	}

	true
}

/// Compare two [`TypeDef`] and return the [`bool`] result.
pub fn type_def(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &TypeDef<PortableForm>,
	b: &TypeDef<PortableForm>,
) -> bool {
	match a {
		TypeDef::Composite(a) => match b {
			TypeDef::Composite(b) => fields(a_types, b_types, &a.fields, &b.fields),
			_ => false,
		},
		TypeDef::Variant(_a) => matches!(b, TypeDef::Variant(_b)),
		// match b {
		// 	TypeDef::Variant(_b) => {
		// 		// TODO: check variants
		// 		// variants(a_types, b_types, a.variants(), b.variants())
		// 		true
		// 	},
		// 	_ => false,
		// },
		TypeDef::Sequence(a) => match b {
			TypeDef::Sequence(b) =>
				untracked_symbol(a_types, b_types, &a.type_param, &b.type_param),
			_ => false,
		},
		TypeDef::Array(a) => match b {
			TypeDef::Array(b) =>
				a.len == b.len && untracked_symbol(a_types, b_types, &a.type_param, &b.type_param),
			_ => false,
		},
		TypeDef::Tuple(a) => match b {
			TypeDef::Tuple(b) => {
				let a = &a.fields;
				let b = &b.fields;

				if a.is_empty() && b.is_empty() {
					return true;
				}
				if a.len() != b.len() {
					return false;
				}

				for (a, b) in a.iter().zip(b.iter()) {
					if !untracked_symbol(a_types, b_types, a, b) {
						return false;
					}
				}

				true
			},
			_ => false,
		},
		TypeDef::Primitive(a) => match b {
			TypeDef::Primitive(b) => a == b,
			_ => false,
		},
		TypeDef::Compact(a) => match b {
			TypeDef::Compact(b) => untracked_symbol(a_types, b_types, &a.type_param, &b.type_param),
			_ => false,
		},
		TypeDef::BitSequence(a) => match b {
			TypeDef::BitSequence(b) =>
				untracked_symbol(a_types, b_types, &a.bit_order_type, &b.bit_order_type)
					&& untracked_symbol(a_types, b_types, &a.bit_store_type, &b.bit_store_type),

			_ => false,
		},
	}
}

/// Compare two [`Field`] and return the [`bool`] result.
pub fn fields(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &[Field<PortableForm>],
	b: &[Field<PortableForm>],
) -> bool {
	if a.is_empty() && b.is_empty() {
		return true;
	}
	if a.len() != b.len() {
		return false;
	}

	for (a, b) in a.iter().zip(b.iter()) {
		if a.name != b.name
			|| a.type_name != b.type_name
			|| a.docs != b.docs
			|| !untracked_symbol(a_types, b_types, &a.ty, &b.ty)
		{
			return false;
		}
	}

	true
}

/// Compare two [`Variant`] and return the [`bool`] result.
pub fn variants(
	a_types: &PortableRegistry,
	b_types: &PortableRegistry,
	a: &[Variant<PortableForm>],
	b: &[Variant<PortableForm>],
) -> bool {
	if a.is_empty() && b.is_empty() {
		return true;
	}
	if a.len() != b.len() {
		return false;
	}

	for (a, b) in a.iter().zip(b.iter()) {
		if a.name != b.name
			|| a.index != b.index
			|| a.docs != b.docs
			|| !fields(a_types, b_types, &a.fields, &b.fields)
		{
			return false;
		}
	}

	true
}
