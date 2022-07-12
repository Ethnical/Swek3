use clap::Parser;
mod contract2interface;

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

#[derive(clap::Subcommand)]
enum Commands {
    /// Extract the contract interface from a Solidity file.
    Contract2interface(Contract2interfaceArgs),
    /// Gas Wei Converter, not implemented yet!
    WeiConverter,
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
    }
}
