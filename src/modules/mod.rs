pub mod logging;

pub mod privatekey {
    use colored::Colorize;
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
        let address = hex::encode(
            &Keccak256::digest(&public_key.serialize_uncompressed()[1..]).as_slice()[12..],
        );
        println!("Address: {}", Colorize::blue(&address[..]));
    }
}

pub mod http_scraper {
    use reqwest;
    use std::fs;
    use std::io::Cursor;
    use std::io::Read;
    use std::path::PathBuf;

    fn check_directory_create_if_not_exist() {
        let path = "swek_output";
        if !PathBuf::from(path).exists() {
            //match the case where the directory already exists
            fs::create_dir(path).unwrap();
        } else {
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

pub mod toml_helper {
    use std::{fs, io};

    use crate::modules::logging::{error, info};
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::path::Path;
    pub use toml::{de::Error, Value};

    #[derive(Debug, Deserialize, Clone)]
    pub struct GlobalConfig {
        RapidForge: HashMap<String, RapidForgeAction>,
    }
    #[derive(Debug, Deserialize, Clone)]
    pub struct RapidForgeAction {
        pub path_store: String,
        pub path_template_solidity: String,
        pub path_template_toml: String,
        pub path_template_interface: String,
    }

    pub fn check_if_folder_is_relative(mut path: String) -> String {
        // Check if the path is relative if yes then replace the ~ by the home directory.
        if path.starts_with("~") {
            match home::home_dir() {
                Some(p) => {
                    let path_home = p.to_str().unwrap();
                    path = path.replace("~", path_home);
                    return path;
                }
                None => {
                    error!("The path {} is relative and swek cannot retrieve the home directory please use absolute path", path);
                    std::process::exit(1);
                }
            }
        }
        path
    }
    fn init_rapidForge(path: String) {
        /// Need to initialise the folders the issues is that rust consider ~ as the new folder and not the home folder...

        // create a cp of the config file in the home directory in the future if there is too much config file we will create a folder for the config file.
        info!("PATH: {}", path);
        let config = read_config_toml(path);

        if Path::new(&check_if_folder_is_relative(
            "~/.config/swek/swek.toml".to_string(),
        ))
        .exists()
        {
            info!("RapidForge already initialised.");
            return;
        } else {
            info!("RapidForge not initialised, creating the folders...");
        }

        info!("First launch of rapidForge, initialising the folders by creating them...");
        let config_audit = parse_toml_rapidforge_normal(config.clone());
        let config_incident = parse_toml_rapidforge_incident(config);

        match fs::create_dir_all(check_if_folder_is_relative(config_audit.path_store)) {
            Ok(path) => println!("Created the audits folder (because first usage)."),
            Err(e) => println!("Error during the first usage  {}", e),
        }

        match fs::create_dir_all(check_if_folder_is_relative(config_incident.path_store)) {
            Ok(path) => println!("Created the incidents folder (because first usage)."),
            Err(e) => println!("Error during the first usage  {}", e),
        }
    }

    pub fn read_config_toml(config_path: String) -> GlobalConfig {
        // let config_path = Path::new("foundry");
        let config = check_if_folder_is_relative(config_path);
        let path = Path::new(&config);
        if !path.is_file() {
            error!("The `swek.toml` is not found at : `{}`. If this is the first launch you can use the command (e.g `swek rapidforge init`) or  also copy the .config/ folder from the github to the correct path ~/swek/.config", path.display());
            std::process::exit(1);
        }
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                panic!("Could not the config file `{}`", path.display());
            }
        };

        let config_parsed: GlobalConfig = match toml::from_str(&contents) {
            Ok(c) => c,
            Err(_) => {
                panic!("Could not parse the config file `{}`", path.display());
            }
        };
        return config_parsed;
    }

    pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
    pub fn parse_toml_rapidforge_incident(config_parsed: GlobalConfig) -> RapidForgeAction {
        config_parsed.RapidForge["incident"].clone()

        //let config_incident: RapidForgeAction = toml::decode(file_contents.parse().unwrap()).unwrap();
    }

    pub fn parse_toml_rapidforge_normal(config_parsed: GlobalConfig) -> RapidForgeAction {
        config_parsed.RapidForge["normal"].clone()

        //let config_incident: RapidForgeAction = toml::decode(file_contents.parse().unwrap()).unwrap();
    }
    //use serde_derive::Deserialize;
}
