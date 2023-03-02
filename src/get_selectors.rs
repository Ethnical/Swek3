use ascii_table::{Align, AsciiTable};
use colored::Colorize;
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
