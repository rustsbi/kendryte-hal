//! Firmware generation module for K230 platform.
//!
//! This module provides functionality to generate encrypted and signed firmware
//! packages for the K230 platform. It supports multiple encryption types including
//! SM4 and AES, along with RSA and SM2 signatures.

use crate::error::{XtaskError, XtaskResult};
use crate::generate::config::{
    ADD_AUTH_DATA, D, E, ID, ID_LEN, INITIAL_AES_IV, INITIAL_AES_KEY, K, MAGIC, N, PRIVATE_KEY,
    PUBLIC_KEY_X, PUBLIC_KEY_Y, SM4_IV, SM4_KEY, VERSION,
};
use aes_gcm::{AeadInPlace, Aes256Gcm, Key, KeyInit, Nonce};
use cipher::BlockEncryptMut;
use num_bigint_dig::BigUint;
use primeorder::PrimeCurveParams;
use rsa::pkcs1v15::SigningKey;
use rsa::signature::{SignatureEncoding, Signer};
use rsa::RsaPrivateKey;
use sha2::{Digest, Sha256};
use sm2::elliptic_curve::ScalarPrimitive;
use sm2::{Scalar, SecretKey, Sm2};
use sm3::Sm3;
use std::str::FromStr;

/// Encryption types supported for firmware.
#[derive(Debug, Default, Clone, Copy)]
pub enum EncryptionType {
    #[default]
    None = 0,
    Sm4 = 1,
    Aes = 2,
}

impl FromStr for EncryptionType {
    type Err = XtaskError;

    /// Parse encryption type from string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "sm4" => Ok(Self::Sm4),
            "aes" => Ok(Self::Aes),
            _ => Err(XtaskError::InvalidEncryptionType),
        }
    }
}

/// Generate firmware with specified data and encryption type.
///
/// This function takes the input data and an encryption type, and generates
/// a firmware package with the appropriate encryption and signature.
pub fn gen_firmware(data: &[u8], encryption: EncryptionType) -> XtaskResult<Vec<u8>> {
    // Prepend version information to the input data
    let mut data_with_version = vec![];
    data_with_version.extend_from_slice(VERSION);
    data_with_version.extend_from_slice(data);

    // Initialize firmware package with magic bytes
    let mut firmware = vec![];
    firmware.extend_from_slice(MAGIC.as_bytes());
    println!("the magic is: {}", MAGIC);

    match encryption {
        EncryptionType::None => {
            println!("----- NO ENCRYPTION + HASH-256 -----");
            // Calculate and store data length (4 bytes, little-endian)
            let data_len = data_with_version.len() as i32;
            let data_len_bytes: [u8; 4] = data_len.to_ne_bytes();
            firmware.extend_from_slice(&data_len_bytes);

            // Store encryption type (4 bytes, little-endian)
            let encryption_bytes: [u8; 4] = (encryption as i32).to_le_bytes();
            firmware.extend_from_slice(&encryption_bytes);
            println!("the encryption type: {}", encryption as i32);

            // Calculate SHA-256 hash of data and add to firmware
            let data_with_version_hash = sha_256(&data_with_version);
            firmware.extend_from_slice(&data_with_version_hash);

            // Add padding to align with firmware format (516 - 32 bytes)
            let padding = vec![0; 516 - 32];
            firmware.extend_from_slice(&padding);

            // Append the actual data
            firmware.extend_from_slice(&data_with_version);
        }
        EncryptionType::Sm4 => {
            println!("----- SM4-CBC + SM2 -----");
            // Encrypt data using SM4 in CBC mode
            type Sm4CbcEnc = cbc::Encryptor<sm4::Sm4>;
            use cbc::cipher::KeyIvInit;
            use sm4::cipher::block_padding::Pkcs7;

            let cipher = Sm4CbcEnc::new(SM4_KEY.into(), SM4_IV.into());
            let ciphertext = cipher.encrypt_padded_vec_mut::<Pkcs7>(&data_with_version);

            // Store encrypted data length and encryption type
            let data_len = ciphertext.len() as i32;
            let data_len_bytes = data_len.to_le_bytes();
            firmware.extend_from_slice(&data_len_bytes);
            let encryption_bytes: [u8; 4] = (encryption as i32).to_le_bytes();
            firmware.extend_from_slice(&encryption_bytes);
            println!("the encryption type: {}", encryption as i32);

            // Signing
            let sk = ScalarPrimitive::from_slice(PRIVATE_KEY)?;
            let secret_key = SecretKey::new(sk);
            let signing_key = sm2::dsa::SigningKey::new(ID, &secret_key)?;

            // Get curve parameters for SM3 hash calculation
            let a = Sm2::EQUATION_A.to_bytes();
            let b = Sm2::EQUATION_B.to_bytes();
            let x_g = Sm2::GENERATOR.0.to_bytes();
            let y_g = Sm2::GENERATOR.1.to_bytes();

            // Prepare Z value for SM2 signature (user ID and curve parameters)
            let mut z = vec![];
            z.extend_from_slice(ID_LEN);
            z.extend_from_slice(ID.as_bytes());
            z.extend_from_slice(&a);
            z.extend_from_slice(&b);
            z.extend_from_slice(&x_g);
            z.extend_from_slice(&y_g);
            z.extend_from_slice(PUBLIC_KEY_X);
            z.extend_from_slice(PUBLIC_KEY_Y);

            let mut hasher = Sm3::new();
            hasher.update(&z);
            let z_a = hasher.finalize();

            // Calculate message hash for signing
            let mut m = vec![];
            m.extend_from_slice(&z_a);
            m.extend_from_slice(&ciphertext);

            let mut hasher = Sm3::new();
            hasher.update(&m);
            let e = hasher.finalize();

            let k = Scalar::from_slice(K)?;
            let signature = signing_key.sign_prehash_with_k(&k, &e)?;

            let r = signature.r().to_bytes();
            let s = signature.s().to_bytes();

            // Combine signature components
            let mut sign = vec![];
            sign.extend_from_slice(&r);
            sign.extend_from_slice(&s);

            // Display signature components for debugging
            display_bytes("sign:", &sign);
            display_bytes("r:", &r);
            display_bytes("s:", &s);

            // Add ID information to firmware
            let id = ID.as_bytes();
            let id_len = id.len() as i32;
            let id_len_bytes = id_len.to_le_bytes();
            firmware.extend_from_slice(&id_len_bytes);

            let mut id_bytes = vec![];
            id_bytes.extend_from_slice(id);
            // Add padding to align with firmware format
            let padding = vec![0; 512 - 32 * 4 - id.len()];
            id_bytes.extend_from_slice(&padding);

            firmware.extend_from_slice(&id_bytes);

            // Add public key components and signature
            firmware.extend_from_slice(PUBLIC_KEY_X);
            firmware.extend_from_slice(PUBLIC_KEY_Y);
            firmware.extend_from_slice(&r);
            firmware.extend_from_slice(&s);

            // Calculate and display SM2 public key hash for verification
            let mut sm2_pub_key = vec![];
            sm2_pub_key.extend_from_slice(&id_len_bytes);
            sm2_pub_key.extend_from_slice(&id_bytes);
            sm2_pub_key.extend_from_slice(PUBLIC_KEY_X);
            sm2_pub_key.extend_from_slice(PUBLIC_KEY_Y);

            let mut hasher = Sm3::new();
            hasher.update(&sm2_pub_key);
            let sm2_pub_key_hash = hasher.finalize();

            display_bytes("the hash value of sm2 puk-key is: ", &sm2_pub_key_hash);

            // Add encrypted data
            firmware.extend_from_slice(&ciphertext);
        }
        EncryptionType::Aes => {
            println!("----- AES-GCM + RSA-2048 -----");
            // Initialize AES-GCM encryption with key and nonce
            let key = Key::<Aes256Gcm>::from_slice(INITIAL_AES_KEY);
            let nonce = Nonce::from_slice(INITIAL_AES_IV);
            let cipher = Aes256Gcm::new(key);
            let mut ciphertext = data_with_version.to_vec();

            // Perform AES-GCM encryption and get authentication tag
            let tag = cipher
                .encrypt_in_place_detached(nonce, ADD_AUTH_DATA, &mut ciphertext)
                .map_err(|e| XtaskError::AesError(e.to_string()))?;
            ciphertext.extend_from_slice(&tag);

            // Store encrypted data length and encryption type
            let data_len = ciphertext.len() as i32;
            let data_len_bytes = data_len.to_le_bytes();
            firmware.extend_from_slice(&data_len_bytes);
            let encryption_bytes: [u8; 4] = (encryption as i32).to_le_bytes();
            firmware.extend_from_slice(&encryption_bytes);
            println!("the encryption type: {}", encryption as i32);

            // Parse RSA key components
            let n = hex::encode(N);
            let n = BigUint::parse_bytes(n.as_bytes(), 16).ok_or(XtaskError::RsaParseError(
                "Failed to parse N for RSA".to_string(),
            ))?;
            let e = u32::from_str_radix(&E[2..], 16)
                .map_err(|_| XtaskError::RsaParseError("Failed to parse E for RSA".to_string()))?;

            let e_bytes = e.to_le_bytes();
            let e = BigUint::from(e);
            let d = hex::encode(D);
            let d = BigUint::parse_bytes(d.as_bytes(), 16).ok_or(XtaskError::RsaParseError(
                "Failed to parse D for RSA".to_string(),
            ))?;

            // Create RSA private key from components
            let private_key = RsaPrivateKey::from_components(
                n.clone(),
                e.clone(),
                d.clone(),
                Vec::new(), // Prime factors omitted for simplicity
            )?;

            display_bytes("tag:", &tag);

            // Generate RSA signature using PKCS#1 v1.5 padding
            let signing_key = SigningKey::<Sha256>::new(private_key);
            let signature = signing_key.sign(&tag).to_vec();

            println!("signature: {}", hex::encode(&signature));
            // Add RSA public key components to firmware
            let n_bytes = n.to_bytes_be();
            firmware.extend_from_slice(&n_bytes);
            firmware.extend_from_slice(&e_bytes);

            // Add RSA signature
            firmware.extend_from_slice(&signature);

            // Calculate and display RSA public key hash for verification
            let mut pub_key = vec![];
            pub_key.extend_from_slice(&n_bytes);
            pub_key.extend_from_slice(&e_bytes);
            let pub_key_hash = sha_256(&pub_key);
            display_bytes("the hash value of RSA puk-key is: ", &pub_key_hash);

            // Add encrypted data
            firmware.extend_from_slice(&ciphertext);
        }
    }

    Ok(firmware)
}

/// Display bytes as hexadecimal string.
fn display_bytes(prefix: &str, bytes: &[u8]) {
    print!("{}", prefix);
    let bytes_hex_str = hex::encode(bytes);
    println!("{}", bytes_hex_str);
}

/// Calculate SHA-256 hash of input data.
fn sha_256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use crate::generate::firmware::{gen_firmware, sha_256, EncryptionType};

    #[test]
    fn test_none_encryption() {
        let firmware: [u8; 113340] =
            include_bytes!("../../../xtask/tests/data/firmware.bin").clone();
        let expect_encrypt_firmware: [u8; 113872] =
            include_bytes!("../../tests/data/firmware_none_encryption.bin").clone();

        let expect_encrypt_firmware_hash = sha_256(&expect_encrypt_firmware);

        let encrypt_firmware =
            gen_firmware(&firmware, EncryptionType::None).expect("Failed to generate firmware");
        let encrypt_firmware_hash = sha_256(&encrypt_firmware);

        assert_eq!(encrypt_firmware_hash, expect_encrypt_firmware_hash);
    }

    #[test]
    fn test_aes_encryption() {
        let firmware: [u8; 113340] =
            include_bytes!("../../../xtask/tests/data/firmware.bin").clone();
        let expect_encrypt_firmware: [u8; 113888] =
            include_bytes!("../../tests/data/firmware_aes_encryption.bin").clone();

        let expect_encrypt_firmware_hash = sha_256(&expect_encrypt_firmware);

        let encrypt_firmware =
            gen_firmware(&firmware, EncryptionType::Aes).expect("Failed to generate firmware");
        let encrypt_firmware_hash = sha_256(&encrypt_firmware);

        assert_eq!(encrypt_firmware_hash, expect_encrypt_firmware_hash);
    }

    #[test]
    fn test_sm4_encryption() {
        let firmware: [u8; 113340] =
            include_bytes!("../../../xtask/tests/data/firmware.bin").clone();
        let expect_encrypt_firmware: [u8; 113888] =
            include_bytes!("../../tests/data/firmware_sm4_encryption.bin").clone();

        let expect_encrypt_firmware_hash = sha_256(&expect_encrypt_firmware);

        let encrypt_firmware =
            gen_firmware(&firmware, EncryptionType::Sm4).expect("Failed to generate firmware");
        let encrypt_firmware_hash = sha_256(&encrypt_firmware);

        assert_eq!(encrypt_firmware_hash, expect_encrypt_firmware_hash);
    }
}
