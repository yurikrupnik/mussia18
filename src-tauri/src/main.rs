// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use serde::__private::de::IdentifierDeserializer;
// use clap::builder::Str;
// use std::fmt::format;
use std::process::{Child, Command};
use std::thread;

mod cluster;
use crate::cluster::install_metallb_helper;
// use crate::cluster::{create_file};
use cluster::run_command;
// use cluster::{create_api};
// use log::info;

// use cluster::create_file;
// use clap::Parser;
// fn create_api(command: &str) {
//     Command::new("sh")
//         .arg("-c")
//         .arg(command)
//         .spawn()
//         .expect("command failed to run");
//         // .expect(format!("{} command failed to run", command).as_str());
// }
fn command_run() -> String {
    let mut cmd = Command::new("task -a")
        // .args(&["/Cargo.toml"])
        // .stdout(Stdio::inherit())
        // .stderr(Stdio::inherit())
        .spawn()
        .expect("ls command failed to stasrt");

    // It's streaming here

    let status = cmd.wait();
    println!("Exited with status {:?}", status);
    String::new()
    // println!("Exited with status {:?}", status);
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// mod clapper {
//     use clap::Parser;
//
//     /// Simple program to greet a person
//     #[derive(Parser, Debug)]
//     #[command(author, version, about, long_about = None)]
//     struct Args {
//         /// Name of the person to greet
//         #[arg(short, long)]
//         name: String,
//
//         /// Number of times to greet
//         #[arg(short, long, default_value_t = 1)]
//         count: u8,
//     }
// }

// fn create_command() {
//     let command = clap::Command::new("cluster-info").subcommand(clap::Command::new("cluster-data"));
// }

// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     name: String,
//
//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

// impl LocalClusters {
//
//     pub fn get_collection<'a>(num: u8) -> &'a str {
//         let se = create_api("kubectx".to_string());
//         // let s = create_file("kubectx".to_string());
//         // const FOOT: Lang = Lang::En {
//         //     singular: "user",
//         //     plural: "users",
//         // };
//         // FOOT.fmt(num)
//     }
// }

/// Api is the same as go tasks https://taskfile.dev/api/
/// Todo try to make it generic without writing
// struct Requires {
//     vars: Vec<String>,
// }
// struct InternalCommand {
//     pub dir: String,
//     pub cmd: String,
//     requires: Requires,
//     shopt: Vec<String>,
//     set: Vec<String>,
//     platforms: Vec<String>,
//     dotenv: Vec<String>,
//     status: Vec<String>,
//     generates: Vec<String>,
//     sources: Vec<String>,
//     aliases: Vec<String>,
//     summary: String,
//     prompt: String,
//     prefix: String,
//     method: String,
//     run: String,
//     task: String,
//     ignore_error: bool,
//     sitent: bool,
//     internal: bool,
//     interactive: bool,
//     // vars: Vec<>
// }

// struct Task {
//     color: bool,
// }

async fn install_clusters_parallel() {
    // let s = create_api(
    //     "ctlptl create cluster k3d --registry=ctlptl-registry".to_string()
    // )?;
    // s.wait_with_output();

    // let t = thread::spawn(move || {
    //     //     create_api("task -a".to_string());
    //     create_api("helm repo add crossplane-stable https://charts.crossplane.io/stable && helm repo update".to_string());
    //     create_api("helm install crossplane --namespace crossplane-system --create-namespace crossplane-stable/crossplane".to_string());
    //
    //     // println!("Finished creating k3d cluster");
    // });
    // thread::spawn(move || {
    //     // some work here
    //     create_api("kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.13.11/config/manifests/metallb-native.yaml".to_string());
    //     // create_api("kind create cluster".to_string());
    //     // println!("Finished creating kind cluster");
    // });
}

#[tauri::command]
fn create_local_workspace(name: &str) {
    let commands = vec![
        "ctlptl create cluster k3d --registry=ctlptl-registry",
        // "helm repo add crossplane-stable https://charts.crossplane.io/stable && helm repo update",
        // "helm install crossplane --namespace crossplane-system --create-namespace crossplane-stable/crossplane",
        "ipconfig getifaddr en0",
    ];

    // Create a vector to hold child processes
    let mut children = vec![];

    // Spawn threads to run each command concurrently
    for command in commands {
        let child = thread::spawn(move || run_command(command.to_string()));
        children.push(child);
    }
}

#[tauri::command]
fn delete_local_workspace(name: &str) {
    format!("delete_local_workspace {}!", name);
    run_command("ctlptl delete cluster k3d".to_string());
}

#[tauri::command]
fn install_metallb(name: &str) {
    format!("delete_local_workspace {}!", name);
    install_metallb_helper();
    //     run_command("kubectl apply -f https://raw.githubusercontent.com/metallb/metallb/v0.13.11/config/manifests/metallb-native.yaml
    // ".to_string());
}
mod cli {
    // use clap::*;
    // #[derive(clap::Parser, Debug)]
    // #[command(author, version, about, long_about = None)]
    // struct Argsa {
    //     /// Name of the person to greet
    //     #[arg(short, long)]
    //     name: String,
    //
    //     /// Number of times to greet
    //     #[arg(short, long, default_value_t = 1)]
    //     count: u8,
    // }
    // /// Simple program to greet a person
    // #[derive(Parser, Debug)]
    // #[command(author, version, about, long_about = None)]
    // struct Args {
    //     /// Name of the person to greet
    //     #[arg(short, long)]
    //     name: String,
    //
    //     /// Number of times to greet
    //     #[arg(short, long, default_value_t = 1)]
    //     count: u8,
    // }

    // fn process_args() {
    //     let args = Argsa::parse();
    //
    //     // let cmd = clap::Command::new("build-configs")
    //     //   .bin_name("build-configs")
    //     //   .subcommand_required(true)
    //     //   .subcommand(clap::command!("generate"));
    //     for _ in 0..args.count {
    //         println!("Hello {}!", args.name)
    //     }
    // }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            create_local_workspace,
            delete_local_workspace,
            install_metallb
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
