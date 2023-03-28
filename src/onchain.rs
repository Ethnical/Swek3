#[allow(non_snake_case)]
use serde_json::Value;
use std::{fs, process};
use terminal_menu::{menu, label, scroll,run,button,mut_menu};
use ethers::{
    abi::AbiEncode,
    providers::{Http, Middleware, Provider},
};
use crossterm::style::Color;
use crate::{modules,onchain, onchain_subcommand, mempool_watcher, eth_call_json};



pub fn get_rpc(chain:String) -> String{
    if chain.contains("Ethereum"){
        return "https://rpc.ankr.com/eth".to_string();
    }else if chain.contains("Binance Smart Chain"){
        return "https://bsc-dataseed.binance.org".to_string();
    } else if chain.contains("Avalanche"){
        return "https://rpc.ankr.com/avalanche".to_string();
    }else if chain.contains("Arbitrum"){
        return "https://rpc.ankr.com/arbitrum".to_string();
    } else if chain.contains("Matic"){
        return "https://polygon-rpc.com".to_string();
    }else if chain.contains("Binance Smart Chain"){
        return "https://bsc-dataseed.binance.org".to_string();
    } else if chain.contains("Fantom"){
        return "https://rpc.ftm.tools/".to_string();
    }
    else if chain.contains("Optimism"){
        return "https://mainnet.optimism.io".to_string();
    }
    else {
        return "Error".to_string();
    }
}
#[derive(clap::Subcommand)]
pub enum Action {
    /// Get a specific offset of the storage.
    storage,
    /// Get the `bytes32(uint256(keccak256('eip1967.proxy.implementation')) - 1)` slot for retrieving the address of the implementation.
    storage_eip1967,
    /// Get the `bytes32(uint256(keccak256('eip1967.proxy.beacon')) - 1)` slot for retrieving the address of the beacon.
    storage_beaconAddress,
     /// Get the `bytes32(uint256(keccak256('eip1967.proxy.admin')) - 1)` slot for retrieving the address of the admin.
    storage_admin,
    /// Get the bytecode of the contract.
    bytecode,
 }



// pub fn get_storage(slot:String, rpc: String) -> String{
//     let slot = H256::from_str("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc");
//     let value = self.provider.get_storage_at(who, slot, block).await?;
//     let addr: H160 = value.into();
// }

pub fn get_bytecode(address:String) -> String{
    let bytecode: String = "".to_string();   
    return bytecode; 
}
pub fn exec_module_onchain(cmd:onchain_subcommand) {
    match cmd{
        onchain_subcommand::bytecode => {
            println!("storage");
        },
        onchain_subcommand::storage => {
            println!("storage");
        },
        onchain_subcommand::storage_eip1967 => {
            println!("storage_eip1967");
        },
        onchain_subcommand::storage_beaconAddress => {
            println!("storage_beaconAddress");
        },
        onchain_subcommand::storage_admin => {
            println!("storage_admin");
        },
        onchain_subcommand::MempoolWatcher(args)=> {
            mempool_watcher::exec_module_watcher_mempool(args.interval);
        },
        onchain_subcommand::JsonToAsm(args)=> {
            eth_call_json::exec_module_JsonToAsm(&args.path);
        },
    }
    process::exit(1);
    let mut _rpc = "".to_string();
    let mut rpc = "".to_string();
    // let menu = menu(vec![
    //     label("Colorize me").colorize(Color::Magenta),
    //     scroll("Me too!", vec!["foo", "bar"]).colorize(Color::Green)
    // ]);
    if rpc.is_empty(){
        let menu = menu(vec![

            // label:
            //  not selectable, useful as a title, separator, etc...
            label("'q' or esc to exit"),
            label("|-----------------------|"),
            label("|Onchain tooling - Menu |"),
            label("|-----------------------|"),
            label(" Please select your RPC:"),
            label(" "),
    
            button(" Ethereum"),
            button(" Binance Smart Chain"),
            button(" Optimism"), 
            button(" Avalanche"), 
            button(" Fantom"),
            button(" Matic"),
    
        ]);
    
    
        run(&menu);
        rpc = get_rpc(mut_menu(&menu).selected_item_name().to_string());
    }
    else{
        rpc = _rpc;
    }
    let provider = Provider::<Http>::try_from(rpc).unwrap();
    
    //let provider = Provider::<Http>::try_from(rpc).unwrap();
  /*   match action {
    Action::bytecode => get_bytecode(address);
    } */
    //
}
