//! Image generation module for K230 platform.

use crate::error::{XtaskError, XtaskResult};
use crate::generate::config::{
    ADD_AUTH_DATA, D, E, ID, ID_LEN, INITIAL_AES_IV, INITIAL_AES_KEY, K, MAGIC, N, PRIVATE_KEY,
    PUBLIC_KEY_X, PUBLIC_KEY_Y, SM4_IV, SM4_KEY, VERSION,
};
use aes_gcm::{AeadInPlace, Aes256Gcm, Key, KeyInit, Nonce, Tag};
use cbc::cipher::KeyIvInit;
use cipher::BlockEncryptMut;
use cipher::block_padding::Pkcs7;
use num_bigint_dig::BigUint;
use primeorder::PrimeCurveParams;
use rsa::RsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::signature::{SignatureEncoding, Signer};
use sha2::{Digest, Sha256};
use sm2::elliptic_curve::ScalarPrimitive;
use sm2::{FieldBytes, Scalar, SecretKey, Sm2};
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

/// Generate a firmware image for the K230 platform.
/// This function creates an image with the specified encryption type.
/// The image includes a header, cryptographic information, and the firmware data.
/// The image is padded to a multiple of 512 bytes.
/// Returns the generated image as a vector of bytes.
pub fn gen_image(firmware: &[u8], encryption: EncryptionType) -> XtaskResult<Vec<u8>> {
    println!("----- Generating image -----");
    let mut image = vec![0; 0x100000];
    image.extend(MAGIC.as_bytes());
    println!("the magic is: {}", MAGIC);

    match encryption {
        EncryptionType::None => handle_none_encryption(&mut image, firmware)?,
        EncryptionType::Sm4 => handle_sm4_encryption(&mut image, firmware)?,
        EncryptionType::Aes => handle_aes_encryption(&mut image, firmware)?,
    }

    if image.len() % 512 != 0 {
        let padding_size = 512 - image.len() % 512;
        image.extend(vec![0; padding_size]);
    }

    Ok(image)
}

/// Prepare the firmware data with version information.
/// This function prepends the version bytes to the firmware data.
/// Returns a new vector containing the version and firmware.
fn prepare_firmware_with_version(firmware: &[u8]) -> Vec<u8> {
    let mut firmware_with_version: Vec<u8> = Vec::with_capacity(VERSION.len() + firmware.len());
    firmware_with_version.extend(VERSION);
    firmware_with_version.extend(firmware);
    firmware_with_version
}

/// Add header information to the image.
/// The header includes the firmware length and encryption type.
/// The length and encryption type are stored as little-endian 32-bit integers.
fn add_header_info(image: &mut Vec<u8>, len: i32, encryption: EncryptionType) {
    image.extend(len.to_le_bytes());
    image.extend((encryption as i32).to_le_bytes());
}

/// Handle the case of no encryption for the firmware image.
/// This function adds a SHA-256 hash of the firmware to the image.
/// The hash is followed by padding and the firmware data itself.
fn handle_none_encryption(image: &mut Vec<u8>, firmware: &[u8]) -> XtaskResult<()> {
    println!("----- NO ENCRYPTION + HASH-256 -----");
    let firmware_with_version = prepare_firmware_with_version(firmware);

    add_header_info(
        image,
        firmware_with_version.len() as i32,
        EncryptionType::None,
    );

    let mut hasher = Sha256::new();
    hasher.update(firmware_with_version.as_slice());
    let hash = hasher.finalize();
    println!("hash: {}", hex::encode(&hash));
    image.extend(hash);
    image.extend(vec![0; 516 - 32]);
    image.extend(firmware_with_version);

    Ok(())
}

/// Handle the case of SM4 encryption for the firmware image.
/// This function encrypts the firmware using SM4-CBC and signs it with SM2.
/// The image includes the signature, public key, and encrypted firmware.
fn handle_sm4_encryption(image: &mut Vec<u8>, firmware: &[u8]) -> XtaskResult<()> {
    println!("----- SM4-CBC + SM2 -----");
    let firmware_with_version = prepare_firmware_with_version(firmware);

    let ciphertext = encrypt_sm4(&firmware_with_version);

    // Add header information.
    add_header_info(image, ciphertext.len() as i32, EncryptionType::Sm4);

    let (signature, r, s) = prepare_sm2_signature(&ciphertext)?;
    println!("signature: {}", hex::encode(&signature));
    println!("r: {}", hex::encode(&r));
    println!("s: {}", hex::encode(&s));
    add_sm2_info(image, r.as_slice(), s.as_slice());
    // Add encrypted data.
    image.extend(ciphertext);

    Ok(())
}

/// Handle the case of AES encryption for the firmware image.
/// This function encrypts the firmware using AES-GCM and signs the tag with RSA-2048.
/// The image includes the RSA signature, public key, and encrypted firmware.
fn handle_aes_encryption(image: &mut Vec<u8>, firmware: &[u8]) -> XtaskResult<()> {
    println!("----- AES-GCM + RSA-2048 -----");
    let firmware_with_version = prepare_firmware_with_version(firmware);

    // Perform AES-GCM encryption.
    let (ciphertext, tag) = encrypt_aes(&firmware_with_version)?;

    println!("tag: {}", hex::encode(&tag));
    // Add header information.
    add_header_info(image, ciphertext.len() as i32, EncryptionType::Aes);

    // Generate and add RSA signature.
    let (signature, n, e) = prepare_rsa_signature(tag)?;
    println!("signature: {}", hex::encode(&signature));
    println!("n: {}", hex::encode(&n));
    println!("e: {}", hex::encode(&e));

    image.extend(n);
    image.extend(e);
    image.extend(signature);
    // Add encrypted data.
    image.extend(&ciphertext);

    Ok(())
}

/// Encrypt the firmware using AES-GCM.
/// Returns the ciphertext and authentication tag.
/// The tag is appended to the ciphertext.
fn encrypt_aes(firmware_with_version: &[u8]) -> XtaskResult<(Vec<u8>, Tag)> {
    let key = Key::<Aes256Gcm>::from_slice(INITIAL_AES_KEY);
    let nonce = Nonce::from_slice(INITIAL_AES_IV);
    let cipher = Aes256Gcm::new(key);

    let mut ciphertext = firmware_with_version.to_vec();
    // Perform AES-GCM encryption and get authentication tag.
    let tag = cipher
        .encrypt_in_place_detached(nonce, ADD_AUTH_DATA, &mut ciphertext)
        .map_err(|e| XtaskError::AesError(e.to_string()))?;
    ciphertext.extend(&tag);
    Ok((ciphertext, tag))
}

/// Prepare an RSA signature for the AES-GCM tag.
/// This function constructs the RSA private key from components and signs the tag.
/// Returns the signature, modulus (n), and exponent (e) as byte vectors.
fn prepare_rsa_signature(tag: Tag) -> XtaskResult<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    // Parse RSA key components.
    let n = hex::encode(N);
    let n = BigUint::parse_bytes(n.as_bytes(), 16).ok_or(XtaskError::RsaParseError(
        "Failed to parse N for RSA".to_string(),
    ))?;

    let e = u32::from_str_radix(&E[2..], 16)
        .map_err(|_| XtaskError::RsaParseError("Failed to parse E for RSA".to_string()))?;
    let e_le_bytes = e.to_le_bytes();
    let e = BigUint::from(e);
    let d = hex::encode(D);
    let d = BigUint::parse_bytes(d.as_bytes(), 16).ok_or(XtaskError::RsaParseError(
        "Failed to parse D for RSA".to_string(),
    ))?;

    // Create RSA private key from components.
    let private_key = RsaPrivateKey::from_components(
        n.clone(),
        e.clone(),
        d.clone(),
        Vec::new(), // Prime factors omitted for simplicity.
    )?;

    // Generate RSA signature using PKCS#1 v1.5 padding.
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let signature = signing_key.sign(&tag).to_vec();

    Ok((signature, n.to_bytes_be(), e_le_bytes.to_vec()))
}

/// Encrypt the firmware using SM4-CBC with PKCS7 padding.
/// Returns the ciphertext as a vector of bytes.
fn encrypt_sm4(firmware_with_version: &[u8]) -> Vec<u8> {
    type Sm4CbcEnc = cbc::Encryptor<sm4::Sm4>;
    let cipher = Sm4CbcEnc::new(SM4_KEY.into(), SM4_IV.into());
    cipher.encrypt_padded_vec_mut::<Pkcs7>(&firmware_with_version)
}

/// Prepare an SM2 signature for the ciphertext.
/// This function calculates the SM3 hash and signs it using the SM2 private key.
/// Returns the signature and its r and s components.
fn prepare_sm2_signature(ciphertext: &[u8]) -> XtaskResult<(Vec<u8>, FieldBytes, FieldBytes)> {
    // Signing.
    let sk = ScalarPrimitive::from_slice(PRIVATE_KEY)?;
    let secret_key = SecretKey::new(sk);
    let signing_key = sm2::dsa::SigningKey::new(ID, &secret_key)?;

    // Get curve parameters for SM3 hash calculation.
    let a = Sm2::EQUATION_A.to_bytes();
    let b = Sm2::EQUATION_B.to_bytes();
    let x_g = Sm2::GENERATOR.0.to_bytes();
    let y_g = Sm2::GENERATOR.1.to_bytes();

    // Prepare Z value for SM2 signature (user ID and curve parameters).
    let mut z = vec![];
    z.extend(ID_LEN);
    z.extend(ID.as_bytes());
    z.extend(&a);
    z.extend(&b);
    z.extend(&x_g);
    z.extend(&y_g);
    z.extend(PUBLIC_KEY_X);
    z.extend(PUBLIC_KEY_Y);

    let mut hasher = Sm3::new();
    hasher.update(&z);
    let z_a = hasher.finalize();

    // Calculate message hash for signing.
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

    let mut signature: Vec<u8> = Vec::with_capacity(r.len() + s.len());
    signature.extend(&r);
    signature.extend(&s);

    Ok((signature, r, s))
}

/// Add SM2-related information to the image.
/// This includes the ID info, public key, and signature components r and s.
fn add_sm2_info(image: &mut Vec<u8>, r: &[u8], s: &[u8]) {
    // Add ID information.
    let id_info = prepare_id_info();
    image.extend(&id_info);

    // Add public key and signature.
    image.extend(PUBLIC_KEY_X);
    image.extend(PUBLIC_KEY_Y);
    image.extend(r);
    image.extend(s);
}

/// Prepare the ID information for the image.
/// The ID info includes the ID length, ID bytes, and padding.
/// Returns the ID info as a vector of bytes.
fn prepare_id_info() -> Vec<u8> {
    let mut id_info = Vec::new();
    let id = ID.as_bytes();
    let id_len_bytes = (id.len() as i32).to_le_bytes();

    id_info.extend(&id_len_bytes);
    id_info.extend(id);
    id_info.extend(vec![0; 512 - 32 * 4 - id.len()]); // Add padding.

    id_info
}

#[cfg(test)]
mod tests {
    use crate::generate::image::{EncryptionType, gen_image};
    use sha2::{Digest, Sha256};

    fn assert_hashes_match(actual: &[u8], expected: &[u8]) {
        let mut hasher = Sha256::new();

        hasher.update(actual);
        let actual_hash = hasher.finalize();

        let mut hasher = Sha256::new();
        hasher.update(expected);
        let expected_hash = hasher.finalize();

        assert_eq!(
            actual_hash,
            expected_hash,
            "Hashes do not match!\nActual:   {}\nExpected: {}",
            hex::encode(&actual_hash),
            hex::encode(&expected_hash)
        );
    }

    #[test]
    fn test_none_encryption() {
        let firmware = include_bytes!("../../../xtask/tests/data/firmware.bin");
        let expected = include_bytes!("../../tests/data/image_none_encryption.img");

        let actual = gen_image(firmware, EncryptionType::None).expect("Encryption failed");

        assert_hashes_match(&actual, expected);
    }

    #[test]
    fn test_aes_encryption() {
        let firmware = include_bytes!("../../../xtask/tests/data/firmware.bin");
        let expected = include_bytes!("../../tests/data/image_aes_encryption.img");

        let actual = gen_image(firmware, EncryptionType::Aes).expect("Encryption failed");

        assert_hashes_match(&actual, expected);
    }

    #[test]
    fn test_sm4_encryption() {
        let firmware = include_bytes!("../../../xtask/tests/data/firmware.bin");
        let expected = include_bytes!("../../tests/data/image_sm4_encryption.img");

        let actual = gen_image(firmware, EncryptionType::Sm4).expect("Encryption failed");

        assert_hashes_match(&actual, expected);
    }
}
