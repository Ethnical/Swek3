use ascii_table::{Align, AsciiTable};
#[allow(non_snake_case)]
use colored::Colorize;
use json::object;
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

use ethers::solc::resolver::Node;
use json::JsonValue;
use solang_parser::pt::FunctionAttribute::BaseOrModifier;
use solang_parser::pt::FunctionAttribute::Visibility;
use solang_parser::pt::FunctionDefinition;
#[allow(non_snake_case)]

pub fn parse_pragma_version(content: &str) -> String {
    let slices: Vec<&str> = content.split("pragma solidity ").collect();
    let slices: Vec<&str> = slices.get(1).unwrap().split(";").collect();
    let res = slices.get(0).unwrap().to_string().replace("^", "");
    res
}
pub fn read_to_string(filename: &str) -> String {
    //Add result String, None

    match fs::read_to_string(filename) {
        Ok(content) => content,
        IsADirectory => get_dir(filename),
        Err(_) => panic!("Cannot read the file quit the program!"),
    }
}
fn is_sol_file(filename: &str) -> bool {
    return filename.contains(".sol");
}

pub fn get_dir(path: &str) -> String {
    let mut source_code: String = String::new();
    for e in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            if is_sol_file(&e.path().display().to_string()) {
                source_code += &(fs::read_to_string(&e.path().display().to_string())
                    .expect("Issue happend during file the reading")
                    + "\n");
                //println!("{}",);
            }
        }
    }
    return source_code;
}
pub fn exec_module_crisk(
    path: &str,
    modifiers_args: &str,
    _crisk_bool: &str,
    _contract_name_arg: String,
    visibility: String,
) {
    // if path.contains("*") {
    //     let folder_path = path.split("*").collect::<Vec<&str>>()[0];
    //     let paths = fs::read_dir(folder_path).unwrap();

    //     for path in paths {
    //         println!("Name: {}", path.unwrap().path().display())
    //     }
    // }

    // Let input be a valid "Standard Solidity Input JSON"
    let contents = read_to_string(path);
    fs::write("/tmp/swek.sol", &contents);

    //println!("File : {}", contents);
    let version = parse_pragma_version(&contents);
    println!("[+] Detected version is  {}", version);

    let x = Node::read("/tmp/swek.sol");
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
    if _crisk_bool == "yes" {
        markdown_display(datatab);
    } else {
        display_modifiers(datatab);
    }

    //let custom_error = format!("Solc version {} not found", version);

    //res.args = vec![String::from("ast-jséon")];
}

fn markdown_display(tab_modifiers: Vec<Vec<String>>) {
    let mut old_contract_name = "".to_string();
    for elem in &tab_modifiers {
        if elem[0] != old_contract_name {
            println!("\nIn the contract `{}` the role `{}` has authority over the functions shown in the diagram below.",elem[0] ,elem[2]);
            old_contract_name = elem[0].clone();
        }
        println!("- `{}` : ", elem[1]);
    }
    println!(
        "Number of match for this modifier is {}",
        tab_modifiers.len()
    );
}
pub fn get_modifiers_from_funcdef(funcdef: FunctionDefinition) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for func_attribute in funcdef.attributes {
        match func_attribute {
            BaseOrModifier(_loc, base) => res.push(base.name.to_string()),
            _ => continue,
        }
    }
    return res;
}

pub fn get_visibility(funcdef: FunctionDefinition) -> String {
    for func_attribute in funcdef.clone().attributes {
        match func_attribute {
            Visibility(e) => return e.to_string(),
            _ => continue,
        }
    }

    return funcdef.ty.to_string();
}

pub fn get_json_from_type_lib(name: String, vec_func_def: Vec<FunctionDefinition>) -> JsonValue {
    //vec_func has all the function of 1 contract. String is the name of the contract

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
                let modifier = get_modifiers_from_funcdef(elem.clone());
                func["library"] = JsonValue::Boolean(true);
                func["visibility"] = JsonValue::String(visibility);
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
                let modifier = get_modifiers_from_funcdef(elem.clone());
                func["visibility"] = JsonValue::String(visibility);
                func["modifiers"].push(modifier.join(","));
                fina_json[name.clone()].push(func);

                //println!("{:?}\n--------------", func["func_name"]);
            }
        }
    }
    return fina_json;
}

pub fn is_not_empty_or_any(modifier: String) -> bool {
    return modifier != "" && modifier != "*";
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
                if is_not_empty_or_any(modifier.clone()) {
                    //Match modifier name with the specify visiblity
                    if is_inside(modifier.clone(), func["modifiers"].to_string())
                        && is_inside(visibility.clone(), func["visibility"].to_string())
                    {
                        //func["modifiers"].to_string().contains(&modifier) {
                        data.push(underscore_has_external(vec![
                            contract.0.to_string(),
                            func["func_name"].to_string(),
                            func["modifiers"].to_string(),
                            func["visibility"].to_string(),
                            func["library"].to_string(),
                        ]));
                    }
                } else if modifier == "" {
                    //Only functions without any modifiers with the specify visibility.
                    //println!("FUNC MODIFIERS => {}", func["modifiers"][0].is_empty());
                    if func["modifiers"][0].is_empty()
                        && is_inside(visibility.clone(), func["visibility"].to_string())
                    {
                        //func["modifiers"].to_string().contains(&modifier) {
                        data.push(underscore_has_external(vec![
                            contract.0.to_string(),
                            func["func_name"].to_string(),
                            func["modifiers"].to_string(),
                            func["visibility"].to_string(),
                            func["library"].to_string(),
                        ]));
                    }
                } else if modifier == "*" {
                    //Match any modifiers with the specify visibility.
                    if is_inside(visibility.clone(), func["visibility"].to_string()) {
                        data.push(underscore_has_external(vec![
                            contract.0.to_string(),
                            func["func_name"].to_string(),
                            func["modifiers"].to_string(),
                            func["visibility"].to_string(),
                            func["library"].to_string(),
                        ]));
                    }
                }
            }
        }
    }
    return data;
}

pub fn underscore_has_external(mut line_tab: Vec<String>) -> Vec<String> {
    if line_tab[1].chars().nth(0).unwrap().to_string() == "_"
        && (line_tab[3].contains("public") || line_tab[3].contains("external"))
    {
        line_tab[0] = format!("{}", line_tab[1]).red().to_string();
        line_tab[1] = format!("{}", line_tab[1]).red().to_string();
        line_tab[2] = format!("{}", line_tab[2]).red().to_string();
        line_tab[3] = format!("{}", line_tab[3]).red().to_string();
        //line_tab[3] = format!("{}", line_tab[0]).red().to_string();
    }
    line_tab
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
