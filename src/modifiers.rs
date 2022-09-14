use ascii_table::{Align, AsciiTable};
use ethers::prelude::artifacts::Node;
use ethers::prelude::artifacts::NodeType;
use ethers::{
    prelude::error::SolcError,
    solc::{CompilerOutput, Solc},
};
use semver::Version;
use serde_json::{json, Value};
#[allow(non_snake_case)]
use std::collections::HashMap;
use std::fmt::Display;
use std::process;

use std::fs;
use std::process::exit;

pub fn exec_module_crisk(
    path: &str,
    modifiers_args: &str,
    crisk_bool: &str,
    contract_name_arg: String,
    list: String,
) {
    // Let input be a valid "Standard Solidity Input JSON"
    let contents = read_to_string(path);

    //println!("File : {}", contents);
    let version = parse_pragma_version(&contents);
    println!("[+] Detected version is  {}", version);

    //let custom_error = format!("Solc version {} not found", version);
    let res = find_or_install_svm_version(version)
        .expect("Error happend during the downloading of the version...");
    //res.args = vec![String::from("ast-json")];

    let json_comp_output = compile_to_ast(res, path);
    if list == "true" {
        display_list_fn(contract_name_list(json_comp_output));
        exit(0x0);
    }
    let map_modifiers = print_modifiers(json_comp_output, contract_name_arg);
    display_modifiers(
        map_modifiers,
        crisk_bool.to_string(),
        modifiers_args.to_string(),
    );
}

/*Return the interface for a sol contract given. */
pub fn compile_to_ast(solc_perso: Solc, filename: &str) -> CompilerOutput {
    let src = read_to_string(filename);
    let solc_config = format!(
        r#"{{
            "language": "Solidity",
            "sources": {{ "input.sol": {{ "content": {} }} }},
            "settings": {{
                "optimizer": {{ "enabled": {} }},
                "libraries": {{ 
                    "input.sol" : {{ {} }} 
                }},
                "outputSelection": {{
                    "*": {{
                        "*": [
                            "evm.bytecode.object", "abi"
                        ],
                    "": [ "*" ] }}
                }}
            }}
        }}"#,
        json!(src),
        false,
        ""
    );

    let datas: Value = serde_json::from_str(&solc_config).unwrap(); //convert in type Value using the serd_json lib
    let s = solc_perso; //Solc::default();
    let json_compiled = s
        .compile(&datas)
        .expect("[-] Failed to compile the contract..");
    return json_compiled;
}

pub fn print_modifiers(
    json_compiled: CompilerOutput,
    contract_name: String,
) -> HashMap<String, Vec<String>> {
    let mut modifiers_hashmap: HashMap<String, Vec<String>> = HashMap::new();

    for elem in json_compiled.sources["input.sol"]
        .clone()
        .ast
        .unwrap()
        .nodes[contractname_to_id(json_compiled, contract_name)] //weeird error with non usize why? I32 was not working?
    .clone()
    .nodes
    {
        if elem.node_type == NodeType::FunctionDefinition {
            let name = elem.other["name"].clone();
            let array_modifiers = get_modifiers_from_func(elem);
            modifiers_hashmap.insert(name.to_string().replace("\"", ""), array_modifiers);
        }
    }
    return modifiers_hashmap;
}

// else if (modifiers_args.is_empty()) {
//     //No modifiers in params
//     modifier_list.push(
//         &modifier["modifierName"]["name"]
//             .to_string()
//             .replace("\"", ""),
//     );
// }

//println!("{:#?}", modifier["modifierName"]["name"]);

/*
for modifier in  {
    println!(modifier["name"]);
}
counter = counter + 1;*/
pub fn display_modifiers(tab: HashMap<String, Vec<String>>, crisk: String, modifiers_args: String) {
    let mut counter_mod = 0;
    let mut data: Vec<Vec<String>> = vec![];
    for elem in tab {
        if !modifiers_args.is_empty() {
            if elem.1.contains(&modifiers_args) {
                if crisk == "true" {
                    println!("- `{}` : ", elem.0);
                }
                if crisk == "false" {
                    data.push(vec![elem.0, format!("{:?}", elem.1)]);
                }

                counter_mod += 1;
            }
        } else {
            if crisk == "true" {
                println!("- `{}` : ", elem.0);
            }
            if crisk == "false" {
                data.push(vec![elem.0, format!("{:?}", elem.1)]);
                //println!("{} | {:?}", elem.0, elem.1);
            }
            counter_mod += 1;
        }
    }

    let mut ascii_table = AsciiTable::default();
    ascii_table
        .column(0)
        .set_header("Function Name")
        .set_align(Align::Left);
    ascii_table
        .column(1)
        .set_header("Modifiers")
        .set_align(Align::Center);

    ascii_table.print(data);
    if !modifiers_args.is_empty() {
        if counter_mod != 0 {
            println!(
                "Number of match for the modifier \"{}\" : {}",
                modifiers_args, counter_mod
            );
            separate();
        } else {
            println!("The number of match is equal to 0, for the modifier {}.\nThis probably an error of typo recheck correctly the modifier syntax! ", modifiers_args);
        }
    } else {
        println!("Number of modifiers inside the contract : {}", counter_mod);
    }
}
pub fn contractname_to_id(json_compiled: CompilerOutput, contract_name: String) -> usize {
    let mut counter = 0;
    for elem in json_compiled.sources["input.sol"]
        .clone()
        .ast
        .unwrap()
        .nodes
    {
        if counter != 0 {
            if elem.other["name"] == contract_name {
                return counter;
            }
        }
        counter += 1;
    }
    if !contract_name.is_empty() {
        println!("Didn't found the contract : \"{}\".\nThis probably an error of typo recheck correctly the contract syntax! ", contract_name);
        separate();
        process::exit(0x1);
    }
    return 0;
}
pub fn contract_name_list(json_compiled: CompilerOutput) -> Vec<Vec<String>> {
    let mut tab_fn: Vec<Vec<String>> = vec![];
    for elem in json_compiled.sources["input.sol"]
        .clone()
        .ast
        .unwrap()
        .nodes
    {
        if elem.other.contains_key("name") {
            tab_fn.push(vec![elem.other["name"].to_string()]);
        }
    }
    return tab_fn;
}

//for elem in json_compiled.sources["input.sol"] {
pub fn separate() {
    println!("--------------------------------------------------------------");
}
pub fn get_modifiers_from_func(elem: Node) -> Vec<String> {
    // return the modifiers for a function given (Node).
    //let mut counter_modifiers = 0;
    let mut modifier_list = vec![];
    for tab in elem.other["modifiers"].as_array() {
        for modifier in tab {
            //
            //modifiers slected in arhgs through cli
            modifier_list.push(
                modifier["modifierName"]["name"]
                    .clone()
                    .to_string()
                    .replace("\"", ""),
            );
        }
    }
    return modifier_list;
}
pub fn parse_pragma_version(content: &str) -> String {
    let slices: Vec<&str> = content.split("pragma solidity ").collect();
    let slices: Vec<&str> = slices.get(1).unwrap().split(";").collect();
    let res = slices.get(0).unwrap().to_string().replace("^", "");
    res
}
pub fn read_to_string(filename: &str) -> String {
    //Add result String, None

    fs::read_to_string(filename).expect(&format!(
        "Something went wrong during the reading of : {}",
        filename
    ))
}

pub fn display_list_fn(data: Vec<Vec<String>>) {
    let mut ascii_table = AsciiTable::default();
    ascii_table
        .column(0)
        .set_header("Name Contract")
        .set_align(Align::Left);

    ascii_table.print(data);
}
//eeeee
pub fn find_or_install_svm_version(version: impl AsRef<str>) -> Result<Solc, SolcError> {
    let version = version.as_ref();
    if let Some(solc) = Solc::find_svm_installed_version(version)? {
        println!("[+] Solc v{} already installed.", version);
        Ok(solc)
    } else {
        println!(
            "[+] Solc v{} not installed. Downloading & Installing the version now, Please wait...",
            version
        );
        Ok(Solc::blocking_install(&version.parse::<Version>()?)?)
    }
}
