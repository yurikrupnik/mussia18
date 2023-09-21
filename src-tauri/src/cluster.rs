use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::future::Future;
use std::io::prelude::*;
use std::process::{Command, Child};
// use std::thread;
// use clap::Arg;

// enum Clis {
//     bun,
//     pnpm,
//     yarn,
//     cargo,
//     go,
//     echo,
//     time,
//     task,
//     minikube,
// }

pub fn create_file() -> std::io::Result<()> {
    let mut file = File::create("aris.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}
pub fn run_command(command: String) -> Child {
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("command failed to run");
    child
}

// Define a struct for the API response (optional)
#[derive(Debug)]
struct ApiResponse {
    // Define the fields you expect in the API response
    // For example:
    // field1: String,
    // field2: i32,
}

use reqwest::blocking::Client;

fn call_api(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = Client::new().get(url).send()?;
    Ok(response.text()?)
}


pub fn install_metallb_helper() {
    let ss = call_api("http://localhost:8080/hey").unwrap();
    // match ss {
    //     Ok(s) => {
            println!("Data is {}", ss);
        // }
        // Err(e) => {
        //     println!("Error is {}", e);
        // }
    // }
    // let s = call_api("http://localhost:8080/heys").expect("aris is error");


    // run_command("kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.13.11/config/manifests/metallb-native.yaml
// ".to_string());
//     get_data().await;
    // run_command("helm install crossplane --namespace crossplane-system --create-namespace crossplane-stable/crossplane".to_string());
    // run_command("helm install metallb metallb/metallb".to_string());
    // run_command("helm repo add argo https://argoproj.github.io/argo-helm".to_string());
    // run_command("helm install my-release argo/argocd-apps".to_string());
    // run_command("helm repo add metallb https://metallb.github.io/metallb".to_string());
    println!("installing metallb")
}

pub async fn create_api(command: String) -> std::io::Result<Child> {
    println!("{command} PASSED!!");
    let child = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .spawn()?;

    Ok(child)
}
