use clap::{Command, Parser};
mod contractinfo_new;
mod eth_call_json;
mod generate_interface_from_contract;
mod mempool_watcher;
mod modules;
mod onchain;
mod private_key_to_address;
mod rapidForge;

#[derive(clap::Parser)]

struct Cli {
    /// Struct that holds the values of the command line arguments.
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Parser)]
struct GenerateInterfaceFromContractArgs {
    #[clap(short, long)]
    /// Path of the Solidity Contract (e.g : --path /src/example.sol).
    path: String,
}
#[derive(clap::Parser)]
pub struct EthCallJsonAsmArgs {
    #[clap(short, long)]
    /// Path of the Json (e.g : --path /src/json.txt).
    path: String,
}
#[derive(clap::Parser, Default)]
pub struct MemPoolArgs {
    #[clap(short, long, default_value_t = 1)]
    /// Interval in milliseconds between each request (e.g : --interval 1000).
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
    /// Path of the Solidity Contract (e.g : --path /src/example.sol). For multiples ".sol" files, please use "." or absolute path folder (e.g: --path /contract/src/).               The path can be an address etherscan (e.g : https://etherscan.io/address/0x1f9840a85d5af5bf1d1762f925bdaddc4201f984)
    path: String,
    #[clap(short, long, default_value = "*")]
    /// Display the function who matched the specify modifier (e.g : onlyOwner). For multiples modifiers use the ","  (e.g :-m onlyOwner, initializer).
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
    /// Display the function who matched the specify visibility (e.g : public). For multiples visiblity use the ","  (e.g : -v public,internal).
    #[clap(short, long, default_value = "external,public,internal,private")]
    visibility: String,
    /// To generate the interface from the solidity code the interface (e.g : --i true).
    #[clap(short, long, default_value = "false")]
    interface: String,
    #[clap(long, default_value = "false")]
    /// To compile the contract on the fly (e.g : --compile_mode true).
    compile_mode: bool,
}

#[derive(clap::Parser)]
struct OnchainArgs {
    #[clap(subcommand)]
    /// Action onchain
    action: onchain::Action,
    #[clap(short, long)]
    /// Address of the contract
    address: String,
    #[clap(short, long)]
    /// RPC of the blockchain
    rpc: String,
}

#[derive(clap::Parser, Default, Clone)]
pub struct FromTemplateArgs {
    /// Link to the contract you want to work on (e.g : --link https://etherscan.io/address/0x1f9840a85d5af5bf1d1762f925bdaddc4201f984)
    #[clap(short, long, default_value = "false")]
    link: String,
    /// The project name (e.g : --project_name wormhole) this is mandatory
    #[clap(short, long)]
    project_name: String,
    /// Te config path (e.g : --config_path /home/user/.config/swek.toml) by default is the .config/swek.toml.
    #[clap(short, long, default_value = "~/.config/swek/swek.toml")]
    config_path: String,
}

#[derive(clap::Parser, Default, Clone)]
pub struct InitArgs {
    /// Te config path (e.g : --config_path /home/user/.config/swek.toml) by default is the .config/swek.toml.
    #[clap(short, long, default_value = ".config/")]
    config_path: String,
}

#[derive(clap::Parser, Clone)]
pub enum RapidForgeSubcommand {
    #[clap(
        name = "incident",
        visible_alias = "inc",
        about = "Create a new incident environnement with the config provided into the `swek.toml`."
    )]
    Incident(FromTemplateArgs),
    #[clap(
        name = "audit",
        visible_alias = "aud",
        about = "Create a new audit environnement with the config provided into the `swek.toml`."
    )]
    Normal(FromTemplateArgs),
    #[clap(
        name = "init",
        visible_alias = "i",
        about = "Generate the config file `swek.toml` at the correct location. This take the `--config_path` option as the path of the folder `.config/`"
    )]
    Init(InitArgs),
    #[clap(
        name = "force",
        visible_alias = "i",
        about = "Force copy the whole config files like `swek.toml` & `incident_templates.sol` at the correct location."
    )]
    ForceCopyConfig(InitArgs),
    //
    //Normal(FromTemplateArgs),
}

#[derive(clap::Parser)]
pub enum onchain_subcommand {
    #[clap(
        name = "mempoolwatcher",
        visible_alias = "mem",
        about = "Watch the mempool in realtime"
    )]
    MempoolWatcher(MemPoolArgs),
    /// Convert Json output from EthCall_debug to Asm.
    #[clap(
        name = "eth-call-to-asm",
        visible_alias = "asm",
        about = "Convert `JSON` from ethcall_debug to ASM"
    )]
    JsonToAsm(EthCallJsonAsmArgs),
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

#[derive(clap::Parser)]
struct PrivateKey2AddressArgs {
    #[clap(short, long)]
    /// Address
    privatekey: String,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Extract the contract interface from a Solidity file.
    GenerateInterfaceFromContract(GenerateInterfaceFromContractArgs),
    /*    /// Gas Wei Converter, not implemented yet!
    WeiConverter
    /// Not implemented yet!
    AnalyzeVerifiedContract(LinkContractArgs),*/
    /// Tools to displays functions list,modifiers, crisk etc..
    ContractInfo(ContractInfoContractArgs),
    /// Tools to call onchain data as storage, bytecode.
    Onchain {
        #[clap(subcommand)]
        command: onchain_subcommand,
    },
    /// RapidForge to create an Audit or Incident environnement faster than ever.
    RapidForge {
        #[clap(subcommand)]
        command: RapidForgeSubcommand,
    },
    /// Convert a private key to an address
    PrivatekeyToAddress(PrivateKey2AddressArgs),
}

fn main() {
    modules::logging::configure_logging(None);

    let _cli = Cli::parse();
    match _cli.command {
        Commands::GenerateInterfaceFromContract(args) => {
            generate_interface_from_contract::exec_module_GenerateInterfaceFromContract(&args.path);
        }
        /*  Commands::WeiConverter => {
            println!("WeiConverter");
        }
        Commands::AnalyzeVerifiedContract(_args) => {
            println!("not implemented yet!");
        }
        */
        Commands::ContractInfo(args) => {
            contractinfo_new::exec_module_crisk(
                &args.path,
                &args.modifiers,
                &args.crisk,
                &args.name_contracts,
                args.contract,
                args.visibility,
                args.interface,
                args.compile_mode,
            );
        }
        Commands::Onchain { command } => {
            onchain::exec_module_onchain(command);
        }
        Commands::PrivatekeyToAddress(args) => {
            private_key_to_address::exec_private_key_to_address(&args.privatekey);
        }
        Commands::RapidForge { command } => {
            rapidForge::exec_module_rapide_forge(command);
        }
    }
}
