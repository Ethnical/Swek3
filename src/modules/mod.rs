pub mod privatekey
{   use colored::Colorize;
    use hex;
    use secp256k1::{PublicKey, SecretKey};
    use sha3::{Digest, Keccak256};
    use std::str::FromStr;

    pub fn private_key_to_address(private_key: &str) {
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
}



pub mod http_scraper {
use std::path::PathBuf;
use std::fs;
use std::io::Cursor;
use std::io::Read;
use reqwest;


    fn check_directory_create_if_not_exist() {
        let path = "swek_output";
        if !PathBuf::from(path).exists() { //match the case where the directory already exists  
            fs::create_dir(path).unwrap();
        }else {  
            fs::remove_dir_all(path).unwrap();
            fs::create_dir(path).unwrap();
        }
    }
    fn blocksec_api(chain: String, address: String) {
        let url = format!(
            "https://extension.blocksec.com/api/v1/source-code/download/?chain={}&address={}",
            chain, address
        );
        
        check_directory_create_if_not_exist();
        let mut response = reqwest::blocking::get(url).unwrap();
        let mut data = Vec::new();
        response.read_to_end(&mut data).unwrap();
        let path = format!("swek_output/{}-{}/", chain, address);
        let target = PathBuf::from(path);
        zip_extract::extract(Cursor::new(data), &target, false);
    }
    pub fn fetchcode(web_path: String) {
        let address = web_path.split("address/").collect::<Vec<&str>>();
        let address = address.get(1);
        let address = match &address {
            Some(x) => x.to_string(),
            None => {
                println!("Something is wrong with the url provided : ({}) thanks to use the following format => https://etherscan.io/address/0x...", web_path);
                panic!("E101");
            }
        };
        if web_path.contains("ether") {
            let chain = "eth".to_string();
            println!("Address : {} Chain : {}", address, chain);
            blocksec_api(chain, address);
        } else if web_path.contains("bsc") {
            let chain = "bsc".to_string();
            println!("Address : {} Chain : {}", address, chain);
            blocksec_api(chain, address);
        }
    }
    
}