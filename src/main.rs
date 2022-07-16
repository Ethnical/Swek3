use clap::Parser;
mod contract2interface;
mod ethCallJson;
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

#[derive(clap::Subcommand)]
enum Commands {
    /// Extract the contract interface from a Solidity file.
    Contract2interface(Contract2interfaceArgs),
    /// Gas Wei Converter, not implemented yet!
    WeiConverter,
    /// Convert Json output from EthCall_debug to Asm.
    Json2asm(EthCallJsonAsmArgs),
    /// Mempool Watcher with a custom intervals in nanoseconds.
    Mempool_watcher(MemPoolArgs),
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
            ethCallJson::exec_module_json_to_asm(&args.path);
        }
        Commands::Mempool_watcher(args) => {
            mempool_watcher::exec_module_watcher_mempool(args.interval);
        }
    }
}
