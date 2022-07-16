use colored::Colorize;
use ethers::{
    abi::AbiEncode,
    providers::{Http, Middleware, Provider},
};
use futures::StreamExt;
use std::env;
use std::time::Duration;
use std::{convert::TryFrom, fmt::format};

pub fn exec_module_watcher_mempool(interval: u128) {
    println!("[+] Watching Mempool with the interval : {}", interval);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(watch_mempool(interval));
    println!("[+] Done!");
}

async fn watch_mempool(interval: u128) {
    let x = env::var("API_KEY").expect("API_KEY not found in ENV please export API_KEY=1337_1337");
    let remotehost_mainnet = format!("https://eth-mainnet.g.alchemy.com/v2/{}", x);
    format!("{}", interval);
    println!("[+] Reading the mempool...");
    let provider = Provider::<Http>::try_from(remotehost_mainnet).unwrap();

    let one_sec = Duration::new(0, u32::try_from(interval).unwrap());
    let provider = provider.interval(one_sec);

    loop {
        let mut watch_tx_stream = provider
            .watch_pending_transactions()
            .await
            .unwrap()
            .transactions_unordered(10);

        for elem in watch_tx_stream.next().await {
            match &elem {
                Ok(tx) => {
                    println!("{}", print_tx(tx));
                }
                Err(e) => {
                    println!("[+] No more transactions {:?}", e);
                    break;
                }
            }
        }
    }
}

fn print_tx(tx: &ethers::core::types::Transaction) -> String {
    //Units::from_wei(tx.value.unwrap(), 18).to_string();
    let mut res = String::from("");
    let mut tx_to = String::from("");
    let mut bloc = String::from("");

    match tx.block_number {
        Some(x) => {
            bloc += &x.to_string().blue().to_string();
        }
        None => {
            bloc = String::from(" Pending.");
        }
    }
    match tx.to {
        Some(x) => {
            tx_to = x.encode_hex();
        }
        None => {
            tx_to = String::from(" CONTRACT CREATION").red().to_string();
        }
    }
    //println!("bloc => {:#?}", tx);
    res += &format!(
        "Tx_hash : {}\nBlock_number :{}\nFrom : {}\nTo : {}\nValue : {} ETH\nGas: {}\nInput_data: {}\n",
        tx.hash.encode_hex(),
        bloc,
        tx.from.encode_hex(),
        tx_to,
        ethers::core::utils::format_ether(tx.value),
        tx.gas,
        tx.input
    );
    res
}
