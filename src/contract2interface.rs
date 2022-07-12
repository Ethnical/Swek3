#[allow(non_snake_case)]
use ethers::prelude::artifacts::NodeType;
use ethers::{
    prelude::error::SolcError,
    solc::{self, Solc},
};
use semver::Version;
use serde_json::{json, Value};

use std::fs;

pub fn exec_module_contract2interface(path: &str) {
    let contents = read_to_string(path);
    let version = parse_pragma_version(&contents);
    println!("[+] Detected version is  {}", version);

    //let custom_error = format!("Solc version {} not found", version);
    let res = find_or_install_svm_version(version)
        .expect("Error happend during the downloading of the version...");
    //res.args = vec![String::from("ast-json")];

    println!(
        "[+] Generated Inteface for the contract {} : \n\n{}\n---------------------------------------------",
        path,
        compile_to_ast(res,path)
    );
}

/*
pub fn main() {
    // Let input be a valid "Standard Solidity Input JSON"
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: {} <input.sol>", args[0]);
    }
    let contents = read_to_string(&args[1]);

    //println!("File : {}", contents);
    let version = parse_pragma_version(&contents);
    println!("[+] Detected version is  {}", version);

    //let custom_error = format!("Solc version {} not found", version);
    let res = find_or_install_svm_version(version)
        .expect("Error happend during the downloading of the version...");
    //res.args = vec![String::from("ast-json")];

    println!(
        "[+] Generated Inteface for the contract {} : \n\n{}\n---------------------------------------------",
        &args[1],
        compile_to_ast(&args[1])
    );
}
*/
/*Return the interface for a sol contract given. */
pub fn compile_to_ast(solc_perso: Solc, filename: &str) -> String {
    let mut res_interface = String::new();
    let mut First_time = true;
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
    let x = s
        .compile(&datas)
        .expect("[-] Failed to compile the contract..");
    //println!("{:?}", x);
    let ast = x.sources["input.sol"].clone().ast.unwrap();
    for elem in ast.nodes {
        if elem.node_type == NodeType::PragmaDirective {
            let s3 = elem.other.get("literals").unwrap()[1]
                .clone()
                .to_string()
                .replace("\"", "");
            let s4 = elem.other.get("literals").unwrap()[2]
                .clone()
                .to_string()
                .replace("\"", "");
            let s = s3 + &s4;
        }
        if elem.node_type == NodeType::ContractDefinition {
            let interface_name = elem.other.get("name").unwrap().as_str().unwrap();

            res_interface += &format!("interface {} {{\n", interface_name);
            res_interface += "/* Variables */\n";
            for nd in elem.nodes {
                if nd.node_type == NodeType::VariableDeclaration {
                    //println!("{:?}", nd.other);
                    let name = nd.other.get("name").unwrap().as_str().unwrap();
                    let type_param = nd
                        .other
                        .get("typeName")
                        .unwrap()
                        .get("typeDescriptions")
                        .unwrap()
                        .get("typeString")
                        .unwrap()
                        .as_str()
                        .unwrap();

                    let visibility = nd.other.get("visibility").unwrap().as_str().unwrap();
                    if visibility != "internal" {
                        res_interface += &format!(
                            "function {} ({} {}) {};\n",
                            name, type_param, "TBD", visibility
                        );
                    }
                }

                //res_interface += format!("Interface {} {", elem.Name);

                if nd.node_type == NodeType::FunctionDefinition {
                    if First_time {
                        res_interface += "/* Functions */\n";
                        First_time = false;
                    }
                    let kind = nd.other.get("kind").unwrap().as_str().unwrap();
                    let mut type_param = "";
                    let mut varname = " TBD";
                    if kind == "function" {
                        let name = nd.other.get("name").unwrap().as_str().unwrap();
                        let param_len = nd
                            .other
                            .get("parameters")
                            .unwrap()
                            .get("parameters")
                            .unwrap()
                            .as_array();

                        if param_len.unwrap().len() > 0 {
                            type_param = nd
                                .other
                                .get("parameters")
                                .unwrap()
                                .get("parameters")
                                .unwrap()[0]
                                .get("typeDescriptions")
                                .unwrap()
                                .get("typeString")
                                .unwrap()
                                .as_str()
                                .unwrap();
                        } else {
                            type_param = "";
                            varname = "";
                        }

                        let visibility = nd.other.get("visibility").unwrap().as_str().unwrap();
                        if visibility != "internal" {
                            res_interface += &format!(
                                "function {} ({}{}) {};\n",
                                name, type_param, varname, visibility
                            );
                        }
                    }
                }
            }
        }
    }
    res_interface += "}";
    res_interface
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
