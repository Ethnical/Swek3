use colored::Colorize;

use hex;
use secp256k1::{PublicKey, SecretKey};
use sha3::{Digest, Keccak256};
use std::str::FromStr;

pub fn exec_private_key_to_address(private_key: &str) {
    let mut private_key = private_key;
    println!("Private key: {}", private_key.yellow());
    if private_key.starts_with("0x") {
        private_key = &private_key[2..];
    }
    let context = secp256k1::Secp256k1::new();
    let secret_key = SecretKey::from_str(private_key).expect("Fail to parse the secret key");
    let public_key = PublicKey::from_secret_key(&context, &secret_key);
    println!(
        "Public key uncompressed: {}",
        Colorize::green(&hex::encode(public_key.serialize_uncompressed())[..])
    );
    println!(
        "Public key compressed: {}",
        Colorize::green(&hex::encode(public_key.serialize())[..])
    );
    // Take the keccak256 of the public key uncompressed. The first two characters should be removed. This is because the 04 is a tag bytes for the ASN.1 OCTET String structure.
    let address =
        hex::encode(&Keccak256::digest(&public_key.serialize_uncompressed()[1..]).as_slice()[12..]);
    println!("Address: {}", Colorize::blue(&address[..]));
}
