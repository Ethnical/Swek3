use colored::Colorize;
use ethers::{
    abi::AbiEncode,
    providers::{Http, Middleware, Provider},
};
use futures::StreamExt;
use std::env;
use std::time::Duration;
use std::{convert::TryFrom, fmt::format};

pub fn exec_module_watcher_mempool(interval: u128) {
    println!("[+] Done!");
}
pub fn download_src(path: &str) {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    Ok(())
}
