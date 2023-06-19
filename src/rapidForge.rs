use crate::modules::toml_helper::*;
use crate::{FromTemplateArgs, RapidForgeSubcommand};
use core::panic;
use ethers::solc::utils::resolve_absolute_library;
use serde_json::error;
use std::env;
use std::fmt::write;
#[allow(non_snake_case)]
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use toml::{de::Error, Value};

use crate::modules::logging::*;
//use forge::init;

fn read_to_file(path: String) -> String {
    let mut file = match File::open(path.clone()) {
        Err(e) => panic!("couldn't open {}: {}", path, e),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
}

fn write_to_file_rec(file_path: &str, data: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create directories: {}", err))?;
    }

    let mut file = File::create(&path)
        .map_err(|err| format!("Couldn't create {}: {}", path.display(), err))?;

    file.write_all(data.as_bytes())
        .map_err(|err| format!("Couldn't write to {}: {}", path.display(), err))?;

    Ok(())
}

fn write_to_file(path: String, content: String) {
    // Write to a file create the file if it doesn't exist.
    let path = Path::new(&path);

    if !path.exists() {
        write_to_file_rec(path.display().to_string().as_str(), "");
    }

    info!("Writing to file: {}", path.display());
    let mut f = match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(check_if_folder_is_relative(path.display().to_string()))
    {
        Err(e) => {
            error!("couldn't open {}: {}", path.display(), e);
            panic!("quit");
        }
        Ok(file) => file,
    };
    f.write_all(content.as_bytes());
}

// let file = match fs::create_dir_all(path.clone()) {
//     Err(e) => {
//         error!("couldn't open {}: {}", path, e);
//         File::create(check_if_folder_is_relative(path)).unwrap()
//     }
//     Ok(file) => File::create(check_if_folder_is_relative(path)).unwrap(),
// };
//file.write_all(content.as_bytes());

//file.write_all(b"Hello, world!").unwrap();

fn copytemplates(
    path_template_solidity: String,
    path_template_toml: String,
    path_store: String,
    path_interfaces: String,
) {
    // write in the file the content of the template.
    let mother_of_all_interfaces = read_to_file(check_if_folder_is_relative(path_interfaces));

    let temp_sol = read_to_file(check_if_folder_is_relative(path_template_solidity));
    let temp_toml = read_to_file(check_if_folder_is_relative(path_template_toml));
    let path_toml = format!("{}/{}", path_store, "foundry.toml");
    let path_sol = format!("{}/script/{}", path_store, "PoC.s.sol");
    let path_interfaces = format!("{}/src/{}", path_store, "mother_of_all_interface.sol");
    write_to_file(path_toml, temp_toml);
    write_to_file(path_sol, temp_sol);
    write_to_file(path_interfaces, mother_of_all_interfaces);

    info!(
        "Template files (`foundry.toml`,`PoC.s.sol`, `mother_of_all_interface.sol`) copied to the new foundry folder {}",
        path_store
    );
}

fn init_foundry(path: &str) {
    let cmd = format!("forge init {} --no-commit", path);
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    let cmd_rm = format!(
        "rm {}/script/Counter.s.sol {}/src/Counter.sol {}/test/Counter.t.sol ",
        path, path, path
    );
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd_rm)
        .output()
        .expect("failed to execute process");
    let cmd = format!("cd {}; forge install --no-commit  OpenZeppelin/openzeppelin-contracts; forge install --no-commit  OpenZeppelin/openzeppelin-contracts-upgradeable",path);
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute the forge install command");
    info!("Foundry repo initialised. Also, Counter.sol and Counter.s.sol removed.");

    //need to use forge to init the folder
}

fn create_and_init_foundry_folder(project_name: String, mut path_store: String) -> String {
    // Return the fullpath of the folder created and init.
    path_store = check_if_folder_is_relative(path_store.to_string());
    let fullpath = format!("{}/{}", path_store, project_name);
    let path = Path::new(&fullpath);

    if !path.exists() {
        // Check if the folder already exists
        fs::create_dir_all(path)
            .expect(format!("The path : {} is not created or valid...", fullpath).as_str());
        info!("New foundry project created at {}", path.display());
        init_foundry(&fullpath);
    } else {
        error!("Folder already with the name `{}` already exists at `{}`. Please change the name of the project...", project_name,path.display());
        std::process::exit(1);
    }
    fullpath
}

fn prepare_template(arg: FromTemplateArgs, is_incident: bool) {
    let config_parsed = read_config_toml(arg.config_path);

    //init_rapidForge(config_parsed.clone());

    if is_incident {
        println!("Incident");
        let config_arg = parse_toml_rapidforge_incident(config_parsed);
        let fullpath =
            create_and_init_foundry_folder(arg.project_name, config_arg.path_store.clone());

        copytemplates(
            config_arg.path_template_solidity,
            config_arg.path_template_toml,
            fullpath.clone(),
            config_arg.path_template_interface,
        );
        exec_vscode(fullpath);
    } else {
        println!("Normal");
        let config_arg = parse_toml_rapidforge_normal(config_parsed);
        let fullpath =
            create_and_init_foundry_folder(arg.project_name, config_arg.path_store.clone());

        copytemplates(
            config_arg.path_template_solidity,
            config_arg.path_template_toml,
            fullpath.clone(),
            config_arg.path_template_interface,
        );
        exec_vscode(fullpath);
    }
}

fn exec_vscode(fullpath: String) {
    let cmd = format!("code {}", fullpath);
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    info!("VSCode opened at {}", fullpath);
}

fn init_rapidForge(path: String, force: bool) {
    /// Need to initialise the folders the issues is that rust consider ~ as the new folder and not the home folder...
    if !Path::new(&path).exists() {
        error!("The config folder is not present at the path `{}`. Please use the `swek rapidforge init --config-path .config/`", path);
        std::process::exit(1);
    }
    if Path::new(&check_if_folder_is_relative(
        "~/.config/swek/swek.toml".to_string(),
    ))
    .exists()
        && force
    {
        info!("RapidForge already initialised quit...");
        return;
    } else {
        info!("Copy all the files to the ~/.config/swek/");
        write_to_file_rec(
            check_if_folder_is_relative("~/.config/swek/swek.toml".to_string()).as_str(),
            read_to_file(check_if_folder_is_relative(path.clone())).as_str(),
        );
        info!("The config file `swek.toml` is successfully copied to ~/.config/swek/");
    }

    copy_dir_all(
        path.clone(),
        &check_if_folder_is_relative("~/.config/swek/".to_string()),
    );

    let config = read_config_toml(check_if_folder_is_relative(
        "~/.config/swek/swek.toml".to_string(),
    ));
    let config_audit = parse_toml_rapidforge_normal(config.clone());
    let config_incident = parse_toml_rapidforge_incident(config);

    match fs::create_dir_all(check_if_folder_is_relative(config_audit.path_store)) {
        Ok(path) => info!("`audits` folder created."),
        Err(e) => println!("Error during the first usage  {}", e),
    };

    match fs::create_dir_all(check_if_folder_is_relative(config_incident.path_store)) {
        Ok(path) => info!("`incidents` folder created."),
        Err(e) => println!("Error during the first usage  {}", e),
    }

    info!("Everything is installed with success at ~/.config/swek/. Use `swek rapid-forge incident --project-name wintermute`  to create a new incident.");
}
pub fn exec_module_rapide_forge(cmd: RapidForgeSubcommand) {
    match cmd {
        RapidForgeSubcommand::Incident(incident) => {
            prepare_template(incident, true);
        }
        RapidForgeSubcommand::Normal(normal) => {
            prepare_template(normal, false);
        }
        RapidForgeSubcommand::Init(initArgs) => {
            init_rapidForge(initArgs.config_path.clone(), true);
        }
        RapidForgeSubcommand::ForceCopyConfig(initArgs) => {
            init_rapidForge(initArgs.config_path.clone(), false);
        }
    }
}
