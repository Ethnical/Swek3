use clap::Parser;
mod contract2interface;
mod contractinfo;
mod eth_call_json;
mod mempool_watcher;
#[derive(clap::Parser)]

struct Cli {
    /// Struct that holds the values of the command line arguments.
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Parser)]
struct Contract2interfaceArgs {
    #[clap(short, long)]
    /// Path of the Solidity Contract (e.g : --path /src/example.sol).
    path: String,
}
#[derive(clap::Parser)]
struct EthCallJsonAsmArgs {
    #[clap(short, long)]
    /// Path of the Json (e.g : --path /src/json.txt).
    path: String,
}
#[derive(clap::Parser, Default)]
struct MemPoolArgs {
    #[clap(short, long, default_value_t = 1)]
    /// Path of the Json (e.g : --path /src/json.txt).
    interval: u128,
}
#[derive(clap::Parser)]
struct LinkContractArgs {
    #[clap(short, long)]
    /// Link to the contract you want to analyze (e.g : --link https://etherscan.io/address/0x1f9840a85d5af5bf1d1762f925bdaddc4201f984).
    link: String,
}
#[derive(clap::Parser, Default)]
struct ContractInfoContractArgs {
    #[clap(short, long)]
    /// Path of the Solidity Contract (e.g : --path /src/example.sol). For multiples ".sol" files, please use "." or absolute path folder (e.g: --path /contract/src/).
    path: String,
    #[clap(short, long, default_value = "*")]
    /// Display the function who matched the specify modifier (e.g : onlyOwner). For multiples modifiers use the ","  (e.g : onlyOwner, initializer).
    modifiers: String,
    #[clap(long, default_value = "false")]
    /// To only display contracts name (e.g --name_contracts true)
    name_contracts: String,
    #[clap(long, default_value = "")]
    /// To not generate crisk markdown (e.g : --crisk true).
    crisk: String,
    #[clap(short, long, default_value = "")]
    /// Select the contract name (e.g : --contract ERC20) by default is empty.
    contract: String,
    #[clap(long, default_value = "false")]
    /// Display the function who matched the specify visibility (e.g : public). For multiples visiblity use the ","  (e.g : public,internal).
    #[clap(short, long, default_value = "external,public,internal,private")]
    visibility: String,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Extract the contract interface from a Solidity file.
    Contract2interface(Contract2interfaceArgs),
    /// Gas Wei Converter, not implemented yet!
    WeiConverter,
    /// Convert Json output from EthCall_debug to Asm.
    Json2asm(EthCallJsonAsmArgs),
    /// Mempool Watcher with a custom intervals in nanoseconds.
    MempoolWatcher(MemPoolArgs),
    /// Not implemented yet!
    AnalyzeVerifiedContract(LinkContractArgs),
    /// Tools to displays functions list,modifiers, crisk etc..
    ContractInfo(ContractInfoContractArgs),
}

fn main() {
    let _cli = Cli::parse();
    match _cli.command {
        Commands::Contract2interface(args) => {
            contract2interface::exec_module_contract2interface(&args.path);
        }
        Commands::WeiConverter => {
            println!("WeiConverter");
        }
        Commands::Json2asm(args) => {
            eth_call_json::exec_module_json_to_asm(&args.path);
        }
        Commands::MempoolWatcher(args) => {
            mempool_watcher::exec_module_watcher_mempool(args.interval);
        }
        Commands::AnalyzeVerifiedContract(_args) => {
            println!("not implemented yet!");
        }
        Commands::ContractInfo(args) => {
            contractinfo::exec_module_crisk(
                &args.path,
                &args.modifiers,
                &args.crisk,
                &args.name_contracts,
                args.contract,
                args.visibility,
            );
        } //  mempool_watcher::exec_module_watcher_mempool(args.interval);
          // }
    }
}
