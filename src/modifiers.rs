#[allow(non_snake_case)]
use ascii_table::{Align, AsciiTable};
use std::collections::HashMap;
use std::fs;

#[macro_use]
use json::object;

use ethers::solc::resolver::Node;
use json::JsonValue;
use serde::__private::de::FlatInternallyTaggedAccess;
use serde::{Deserialize, Serialize};
use serde_json::{to_string, Result, Value};
use solang_parser::pt::FunctionAttribute::BaseOrModifier;
use solang_parser::pt::FunctionAttribute::Visibility;
use solang_parser::pt::{Base, FunctionDefinition, Identifier};
#[allow(non_snake_case)]
use std::process;
use std::process::exit;

use std::env;

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
pub fn exec_module_crisk(
    path: &str,
    modifiers_args: &str,
    crisk_bool: &str,
    contract_name_arg: String,
    visibility: String,
) {
    // Let input be a valid "Standard Solidity Input JSON"
    let contents = read_to_string(path);

    //println!("File : {}", contents);
    let version = parse_pragma_version(&contents);
    println!("[+] Detected version is  {}", version);

    let x = Node::read(path);
    let test = &x.unwrap();

    let contracts = &test.get_data().contracts;

    let libraries = &test.get_data().libraries;
    let data = r#"{
        "bloc_name":[]
        }"#;
    let mut json_glob = json::parse(data).unwrap();

    let mut _hashmap_librairies: HashMap<String, HashMap<String, Vec<&str>>> = HashMap::new();

    for elem in contracts {
        json_glob["bloc_name"].push(get_json_from_type(
            elem.name.clone(),
            elem.functions.clone(),
        ));
    }

    for elem in libraries {
        //libraries
        json_glob["bloc_name"].push(get_json_from_type_lib(
            elem.name.clone(),
            elem.functions.clone(),
        ));
    }

    //println!("{}", json_glob.pretty(1));
    let datatab = create_display_tab(json_glob, visibility, modifiers_args.to_string());
    display_modifiers(datatab);
    //let custom_error = format!("Solc version {} not found", version);

    //res.args = vec![String::from("ast-jséon")];
}
pub fn tesdf(funcdef: FunctionDefinition) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for func_attribute in funcdef.attributes {
        match func_attribute {
            BaseOrModifier(loc, base) => res.push(base.name.to_string()),
            _ => continue,
        }
    }
    return res;
}

pub fn get_visibility(funcdef: FunctionDefinition) -> String {
    let mut res: String = String::new();
    for func_attribute in funcdef.clone().attributes {
        match func_attribute {
            Visibility(e) => return e.to_string(),
            _ => continue,
        }
        /*match func_attribute.clone() {
            Visibility => println!("sfdsfsdf {:?}", func_attribute),
            _ => continue,
        }*/
        //println!("HERE => {:?}", func_attribute);
    }

    return funcdef.ty.to_string();
}
pub fn get_name_from_identifier(iden: &Option<Identifier>) -> String {
    match iden {
        Some(_ide) => return iden.clone().unwrap().name,
        None => return String::from(""),
    }
}
pub fn get_name_from_base(base: &Base) -> Vec<&String> {
    let mut modifiers_tab = vec![];
    for modifier_name in &base.name.identifiers {
        if &modifier_name.name != "" {
            // println!("Modifier => {:?}",&modifier_name.name);
            modifiers_tab.push(&modifier_name.name);
        }
    }
    return modifiers_tab;
}
pub fn get_name_from_base_json(base: Base) -> String {
    for modifier_name in &base.name.identifiers {
        if &modifier_name.name != "" {
            return modifier_name.name.clone();
        }
    }
    return "".to_string();
}

pub fn get_json_from_type_lib(name: String, vec_func_def: Vec<FunctionDefinition>) -> JsonValue {
    //vec_func has all the function of 1 contract. String is the name of the contract

    let data = r#"{
        "funcdef": {"contract_name": "", "function_name": "","modifiers":[],"visibility":"", "isLibrary": ""}
        }"#;
    let mut func;

    let mut fina_json = json::parse(&format!("{{\"{}\": []}}", name)).unwrap();
    //        let parsed = json::parse().unwrap();

    for elem in vec_func_def {
        // println!("{:?}\n-----------", elem);
        match elem.name {
            None => continue,
            Some(ref iden) => {
                func = object! {
                func_name:iden.name.clone(),
                    modifiers: [],
                    library: false,
                    visibility: "".to_string()
                };
                let visibility = get_visibility(elem.clone());
                let modifier = tesdf(elem.clone());
                func["library"] = JsonValue::Boolean((true));
                func["visibility"] = JsonValue::String((visibility));
                func["modifiers"].push(modifier.join(","));
                fina_json[name.clone()].push(func);

                //println!("{:?}\n--------------", func["func_name"]);
            }
        }
    }
    return fina_json;
}

pub fn get_json_from_type<'a>(name: String, vec_func_def: Vec<FunctionDefinition>) -> JsonValue {
    //vec_func has all the function of 1 contract. String is the name of the contract

    let data = r#"{
        "funcdef": {"contract_name": "", "function_name": "","modifiers":[],"visibility":"", "isLibrary": ""}
        }"#;
    let mut func;

    let mut fina_json = json::parse(&format!("{{\"{}\": []}}", name)).unwrap();
    //        let parsed = json::parse().unwrap();

    for elem in vec_func_def {
        // println!("{:?}\n-----------", elem);
        match elem.name {
            None => continue,
            Some(ref iden) => {
                func = object! {
                func_name:iden.name.clone(),
                    modifiers: [],
                    library: false,
                    visibility: "".to_string()
                };
                let visibility = get_visibility(elem.clone());
                let modifier = tesdf(elem.clone());
                func["visibility"] = JsonValue::String((visibility));
                func["modifiers"].push(modifier.join(","));
                fina_json[name.clone()].push(func);

                //println!("{:?}\n--------------", func["func_name"]);
            }
        }
    }
    return fina_json;
}
pub fn get_modifier_from_vec_def<'a>(
    name: &'a String,
    vec_func_def: &'a Vec<FunctionDefinition>,
) -> (&'a String, HashMap<String, Vec<&'a str>>) {
    let mut modifiers_hashmap: HashMap<String, Vec<&str>> = HashMap::new();
    for funcdef in vec_func_def {
        let mut tmp_array: Vec<&str> = vec![];
        let ident = &funcdef.name;

        for elem_attrib in &funcdef.attributes {
            match elem_attrib {
                BaseOrModifier(_loc, base) => {
                    if base.args == None {
                        tmp_array.push(get_name_from_base(base)[0]);
                    }
                }
                _ => continue,
            }
        }
        if get_name_from_identifier(ident) != "" {
            //remove the empty function "" => [] at the begininning everytime.
            modifiers_hashmap.insert(get_name_from_identifier(ident), tmp_array);
        }
    }
    //display_modifiers((name,modifiers_hashmap.clone()));
    return (name, modifiers_hashmap);
    //println!("Modifiers are :{:?}\n------------------------", modifiers_hashmap);
}

pub fn is_inside(args_str: String, string_to_check: String) -> bool {
    let tab = args_str.split(",");
    for str_split in tab {
        if string_to_check.contains(str_split) {
            return true;
        }
    }
    return false;
}

pub fn create_display_tab(
    tab: JsonValue,
    visibility: String,
    modifier: String,
) -> Vec<Vec<String>> {
    let mut _counter_mod = 0;
    let mut data: Vec<Vec<String>> = vec![];

    for bloc_name in tab["bloc_name"].members() {
        for contract in bloc_name.entries() {
            for func in contract.1.members() {
                if modifier.len() > 0 && visibility.len() == 0 {
                    if is_inside(modifier.clone(), func["modifiers"].to_string()) {
                        //func["modifiers"].to_string().contains(&modifier) {
                        data.push(vec![
                            contract.0.to_string(),
                            func["func_name"].to_string(),
                            func["modifiers"].to_string(),
                            func["visibility"].to_string(),
                            func["library"].to_string(),
                        ]);
                    }
                } else if modifier.len() > 0 && visibility.len() > 0 {
                    if is_inside(modifier.clone(), func["modifiers"].to_string())
                        && is_inside(visibility.clone(), func["visibility"].to_string())
                    {
                        //func["modifiers"].to_string().contains(&modifier) {
                        data.push(vec![
                            contract.0.to_string(),
                            func["func_name"].to_string(),
                            func["modifiers"].to_string(),
                            func["visibility"].to_string(),
                            func["library"].to_string(),
                        ]);
                    }
                } else if modifier.len() == 0 && visibility.len() > 0 {
                    if (is_inside(visibility.clone(), func["visibility"].to_string())) {
                        //func["modifiers"].to_string().contains(&modifier) {
                        data.push(vec![
                            contract.0.to_string(),
                            func["func_name"].to_string(),
                            func["modifiers"].to_string(),
                            func["visibility"].to_string(),
                            func["library"].to_string(),
                        ]);
                    }
                } else {
                    data.push(vec![
                        contract.0.to_string(),
                        func["func_name"].to_string(),
                        func["modifiers"].to_string(),
                        func["visibility"].to_string(),
                        func["library"].to_string(),
                    ]);
                }
            }
        }
    }
    return data;
}

pub fn display_modifiers(data: Vec<Vec<String>>) {
    let mut ascii_table = AsciiTable::default();
    ascii_table
        .column(0)
        .set_header("Contract Name")
        .set_align(Align::Left);
    ascii_table
        .column(1)
        .set_header("Function")
        .set_align(Align::Left);
    ascii_table
        .column(2)
        .set_header("Modifier(s)")
        .set_align(Align::Left);
    ascii_table
        .column(3)
        .set_header("Visibility")
        .set_align(Align::Left);
    ascii_table
        .column(4)
        .set_header("IsLibrary")
        .set_align(Align::Left);

    ascii_table.print(data);
}
