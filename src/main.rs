use clap::Parser;
mod contract2interface;
mod eth_call_json;
mod mempool_watcher;
mod modifiers;
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
struct ModifiersContractArgs {
    #[clap(short, long)]
    /// Path of the Solidity Contract (e.g : --path /src/example.sol).
    path: String,
    #[clap(short, long, default_value = "")]
    /// select a modifier.
    modifiers: String,
    #[clap(long, default_value = "false")]
    /// To not generate crisk markdown (e.g : --crisk true) by default is false.
    crisk: String,
    #[clap(short, long, default_value = "")]
    /// Select the contract name (e.g : --contract ERC20) by default is empty.
    contract: String,
    #[clap(long, default_value = "false")]
    /// Display the list of fn inside the sol file (e.g : --list true by default is false.
    #[clap(short, long, default_value = "false")]
    list: String,
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
    Contract_info(ModifiersContractArgs),
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
        Commands::Contract_info(args) => {
            modifiers::exec_module_crisk(
                &args.path,
                &args.modifiers,
                &args.crisk,
                args.contract,
                args.list,
            );
        } //  mempool_watcher::exec_module_watcher_mempool(args.interval);
          // }
    }
}
