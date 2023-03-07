use ascii_table::{Align, AsciiTable};
use colored::Colorize;
use ethers::{
    core::types::Address,
    providers::{Http, Middleware, Provider},
};
use heimdall::decompile::util;
use heimdall_common::ether::{
    evm::opcodes::opcode,
    signatures::{resolve_function_signature, ResolvedFunction},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn exec_get_selectors(bytecode: &str) {
    // A lot of code come from heimdall-rs
    let output = get_opcodes_from_bytecode(bytecode);

    let mut ascii_table = AsciiTable::default();
    ascii_table
        .column(0)
        .set_header(Colorize::yellow("Selectors").to_string())
        .set_align(Align::Left);
    ascii_table
        .column(1)
        .set_header(Colorize::yellow("Known signatures").to_string())
        .set_align(Align::Left);

    //println!("{:#?}", util::find_function_selectors(output));
    let selectors = util::find_function_selectors(output);
    let mut data: Vec<Vec<String>> = vec![];

    let resolved_functions = resolve_signature_from_selectors(&selectors);
    // Update to match ascii_table needs
    for selector in selectors {
        let functions = match resolved_functions.lock().unwrap().get(&selector) {
            Some(func) => func.clone(),
            None => continue,
        };
        let mut signatures = "".to_string();
        for function in functions {
            signatures = function.signature + ",";
        }
        signatures.pop(); // Remove the last ","

        data.push(vec![selector, signatures.to_string()]);
    }
    ascii_table.print(data);
}

pub fn exec_get_selectors_onchain(address: &str, rpc: &str) {
    // create new runtime block
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // We are decompiling a contract address, so we need to fetch the bytecode from the RPC provider.
    let contract_bytecode = rt.block_on(async {
        // make sure the RPC provider isn't empty
        if rpc.len() <= 0 {
            println!("rpc url is not set");
            std::process::exit(1);
        }

        // create new provider
        let provider = match Provider::<Http>::try_from(rpc) {
            Ok(provider) => provider,
            Err(_) => {
                println!(
                    "{}",
                    &format!("failed to connect to RPC provider '{}' .", rpc)
                );
                std::process::exit(1)
            }
        };

        // safely unwrap the address
        let address = match address.parse::<Address>() {
            Ok(address) => address,
            Err(_) => {
                println!("{}", &format!("failed to parse address '{}' .", address));
                std::process::exit(1)
            }
        };

        // fetch the bytecode at the address
        let bytecode_as_bytes = match provider.get_code(address, None).await {
            Ok(bytecode) => bytecode,
            Err(_) => {
                println!(
                    "{}",
                    &format!("failed to fetch bytecode from '{}' .", address)
                );
                std::process::exit(1)
            }
        };
        bytecode_as_bytes.to_string().replacen("0x", "", 1)
    });

    exec_get_selectors(&contract_bytecode);
}

// get_opcodes_from_bytecode convert the bytecode to opcodes
fn get_opcodes_from_bytecode(bytecode: &str) -> String {
    let mut program_counter = 0;
    let mut output: String = String::new();
    // Iterate over the bytecode, disassembling each instruction.
    let byte_array = bytecode
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    while program_counter < byte_array.len() {
        let operation = opcode(&byte_array[program_counter]);
        let mut pushed_bytes: String = String::new();

        if operation.name.contains("PUSH") {
            let byte_count_to_push: u8 = operation.name.replace("PUSH", "").parse().unwrap();

            pushed_bytes = match byte_array
                .get(program_counter + 1..program_counter + 1 + byte_count_to_push as usize)
            {
                Some(bytes) => bytes.join(""),
                None => break,
            };
            program_counter += byte_count_to_push as usize;
        }

        output.push_str(
            format!("{} {} {}\n", program_counter, operation.name, pushed_bytes).as_str(),
        );
        program_counter += 1;
    }
    output
}

// resolve_signature_from_selectors get all possible signature from the selector. The website api.openchain.xyz is used.
fn resolve_signature_from_selectors(
    selectors: &Vec<String>,
) -> Arc<Mutex<HashMap<String, Vec<ResolvedFunction>>>> {
    let resolved_functions: Arc<Mutex<HashMap<String, Vec<ResolvedFunction>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let mut threads = Vec::new();

    // resolve the signature
    for selector in selectors.clone() {
        // create a new thread for each selector
        let function_clone = resolved_functions.clone();
        threads.push(thread::spawn(move || {
            match resolve_function_signature(&selector) {
                Some(function) => {
                    let mut _resolved_functions = function_clone.lock().unwrap();
                    _resolved_functions.insert(selector, function);
                }
                None => {}
            }
        }));
    }

    // wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    resolved_functions
}
