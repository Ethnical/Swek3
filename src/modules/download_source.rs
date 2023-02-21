use colored::Colorize;
use ethers::{
    abi::AbiEncode,
    providers::{Http, Middleware, Provider},
};
use futures::StreamExt;
use std::env;
use std::time::Duration;
use std::{convert::TryFrom, fmt::format};


pub fn blocksec_api(chain: String, address: String) {
    let url = format!(
        "https://extension.blocksec.com/api/v1/source-code/download/?chain={}&address={}",
        chain, address
    );
    match fs::create_dir("swek_output") {
        Ok(_) => todo!(),
        Err(_) => println!("Error cannot create folder on the disk!"),
    };

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
            println!("Something is wrong with the url provided thanks to use the following format => https://etherscan.io/address/0x...");
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
