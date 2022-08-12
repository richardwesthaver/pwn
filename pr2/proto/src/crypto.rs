//! crypto.rs --- 
use crate::error::*;
use serde::{Serialize, Deserialize};
use chacha20poly1305::{Key, XChaCha20Poly1305, XNonce, KeyInit, aead::Aead};

// use x25519_dalek::EphemeralSecret;
pub const X25519_PRIVATE_KEY_SIZE: usize = 32;
pub const X25519_PUBLIC_KEY_SIZE: usize = 32;

pub const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;
pub const XCHACHA20_POLY1305_KEY_SIZE: usize = 32;

pub const ED25519_PUBLIC_KEY_SIZE: usize = 32;
pub const ED25519_PRIVATE_KEY_SIZE: usize = 32;
pub const ED25519_SIGNATURE_SIZE: usize = 64;

// all keys are 256-bit values
pub type SharedKey = [u8; 32];
pub type PublicKey = [u8; 32];
pub type PrivateKey = [u8; 32];

// generate a SharedKey from a private_key and peer's public_key
pub fn get_shared_key(private_key: PrivateKey, public_key: PublicKey) -> SharedKey {
    let own_secret = x25519_dalek::StaticSecret::from(private_key);
    let their_public = x25519_dalek::PublicKey::from(public_key);

    own_secret.diffie_hellman(&their_public).to_bytes()
}

pub trait Encrypt<A> {
  fn encrypt() -> Result<Encrypted>;
}

pub trait Decrypt<A> {
  fn decrypt() -> Result<Vec<u8>>;
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Encrypted {
  nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
  ciphertext: Vec<u8>,
}

impl Encrypted {
    pub fn decrypt(&self, shared_key: SharedKey) -> Result<Vec<u8>> {
        let key = Key::from_slice(&shared_key);
        let cipher = XChaCha20Poly1305::new(key);
        let nonce = XNonce::from_slice(&self.nonce);

        let plaintext = cipher.decrypt(nonce, self.ciphertext.as_ref())?;

        Ok(plaintext)
    }

    pub fn from_byte_array<T: AsRef<[u8]>, CR: rand::Rng + rand::CryptoRng>(
        shared_key: SharedKey,
        data: T,
        rng: &mut CR,
    ) -> Result<Encrypted> {
        let mut raw_nonce = [0u8; XCHACHA20_POLY1305_NONCE_SIZE];
        rng.fill(&mut raw_nonce);

        let key = Key::from_slice(&shared_key);
        let nonce = XNonce::from_slice(&raw_nonce);
        let cipher = XChaCha20Poly1305::new(key);

        let ciphertext = cipher.encrypt(nonce, data.as_ref())?;

        Ok(Encrypted {
            nonce: raw_nonce,
            ciphertext,
        })
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CryptoContext {
  pub public_key: PublicKey,
  pub shared_key: SharedKey,
}

impl CryptoContext {
    pub fn new<CR: rand::Rng + rand::CryptoRng>(
        rng: &mut CR,
        server_public_key: PublicKey,
    ) -> Self {
        let mut raw_private_key: PublicKey = Default::default();
        rng.fill_bytes(&mut raw_private_key);

        // TODO use EphemeralSecret when x25519-dalek upgrades rand dependency, ref https://github.com/dalek-cryptography/x25519-dalek/issues/65 https://github.com/dalek-cryptography/x25519-dalek/pull/64
        let private_key = x25519_dalek::StaticSecret::from(raw_private_key);
        let public_key = x25519_dalek::PublicKey::from(&private_key).to_bytes();
        let x25519_public_key = x25519_dalek::PublicKey::from(server_public_key);

        let shared_key = private_key.diffie_hellman(&x25519_public_key).to_bytes();

        CryptoContext {
            public_key,
            shared_key,
        }
    }
}
