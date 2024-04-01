// crates.io
use crypto_secretbox::{aead::AeadMut, KeyInit, XSalsa20Poly1305};
use scrypt::Params;
// subcryptor
use crate::{constant::*, prelude::*};

struct Scrypt {
	params: Params,
	salt: Vec<u8>,
}
impl Scrypt {
	const LEN: usize = 32 + (3 * 4);
	const N: u32 = 1 << 15;
	const P: u32 = 1;
	const PARAMS_LEN: usize = 32;
	const R: u32 = 8;

	fn from_bytes(bytes: &[u8]) -> Self {
		let salt = bytes[0..32].to_vec();
		let n = u32::from_le_bytes(array_bytes::slice2array_unchecked(&bytes[32..36]));
		let p = u32::from_le_bytes(array_bytes::slice2array_unchecked(&bytes[36..40]));
		let r = u32::from_le_bytes(array_bytes::slice2array_unchecked(&bytes[40..44]));

		if n != Self::N || p != Self::P || r != Self::R {
			panic!("Invalid injected scrypt params found");
		}

		Self { params: Params::new(n.ilog2() as _, r, p, Self::PARAMS_LEN).unwrap(), salt }
	}
}
#[test]
fn default_scrypt_should_work() {
	assert!(Params::new(Scrypt::N.ilog2() as _, Scrypt::R, Scrypt::P, Scrypt::PARAMS_LEN).is_ok());
}

/// Decrypt the encrypted keystore.
pub fn decrypt_keystore<S>(
	passphrase: &[u8],
	encrypted: &[u8],
	types: &[S],
) -> Result<[u8; SECRET_KEY_LEN]>
where
	S: AsRef<str>,
{
	if !types.iter().any(|t| t.as_ref() == "xsalsa20-poly1305") {
		Err(Error::UnsupportedEncryptionType)?;
	}

	let Scrypt { params, salt } = Scrypt::from_bytes(encrypted);
	let mut password_hash = [0; 32];

	// TODO: check if it's `scrypt`.
	scrypt::scrypt(passphrase, &salt, &params, &mut password_hash)?;

	let encrypted = &encrypted[Scrypt::LEN..];
	// TODO: use `Key::from_array` once crypto_secretbox updates its dependency.
	let mut secret_box =
		XSalsa20Poly1305::new_from_slice(&password_hash).map_err(error::CryptoSecretBox::Cipher)?;
	let cipher = &encrypted[XSalsa20Poly1305::NONCE_SIZE..];
	let nonce = &encrypted[..XSalsa20Poly1305::NONCE_SIZE];
	let decrypted =
		secret_box.decrypt(nonce.into(), cipher).map_err(error::CryptoSecretBox::General)?;

	array_bytes::slice2array(&decrypted[SEED_OFFSET..SEED_OFFSET + SECRET_KEY_LEN])
		.map_err(Error::ArrayBytes)
}
#[test]
fn decrypt_keystore_should_work() {
	let encrypted = "0xa5a3b74af6d77ff92d9c9c2ec41e499f72f9631aeff493e138b03876994e16ef008000000100000008000000ae219a007b8b1f122579e6c52c65a1aaf5c0b075d6aa61d044eeb3127d97d877b923830e58508a3b401e9ef8f558faae83bbcb914b138e8d1e7ef8b4c46af969a636e411c7b7f3f32bb039ab29c714ab1bbe863bfc46dfd12bbedbb5c4ae5839ba080e5b817df2da529e62a24489d31b3faf1d48a10df2e3a811d3ae09d005087d411c228f84e72ef3444d710f69b990ea65d2e83f173b403945f4b4b9";
	let alice_secret_key = "0x98319d4ff8a9508c4bb0cf0b5a78d760a0b2082c02775e6e82370816fedfff48925a225d97aa00682d6a59b95b18780c10d7032336e88f3442b42361f4a66011";

	assert_eq!(
		array_bytes::bytes2hex(
			"0x",
			decrypt_keystore(
				b"456123",
				&array_bytes::hex2bytes_unchecked(encrypted),
				&["xsalsa20-poly1305", "scrypt"]
			)
			.unwrap()
		),
		alice_secret_key
	);
}
