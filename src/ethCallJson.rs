#[allow(non_snake_case)]
use serde_json::Value;
use std::fs;

fn json_to_asm(path: &str) {
    let mut line = String::new();
    let file = fs::read_to_string(path).expect("Unable to read the file");
    let json_data: Value = serde_json::from_str(&file).expect("Unable to parse the json");
    //println!("{}", json_data["result"]["structLogs"]);
    for elem in json_data["result"]["structLogs"].as_array().unwrap() {
        line += &format!(
            "{} {}\n",
            elem["op"].to_string().replace("\"", ""),
            elem["stack"]
        );
    }
    println!("{}", line);
}
pub fn exec_module_json_to_asm(path: &str) {
    println!("[+] Extract Asm for the file : {}", path);
    json_to_asm(path);
    println!("[+] Done!");
}
