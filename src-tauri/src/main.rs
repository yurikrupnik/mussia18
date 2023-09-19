// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;
use std::process::{Command,};
use std::thread;
use clap::builder::Str;
use serde::__private::de::IdentifierDeserializer;

mod cluster;
use cluster::create_api;
use log::info;
use crate::cluster::create_file;

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
        .expect("ls command failed to start");

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

fn create_command() {
    let command = clap::Command::new("cluster-info")
        .subcommand(clap::Command::new("cluster-data"))
        ;
}

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
struct InternalCommand {

}


struct Task {
    color: bool,

}

fn install_clusters_parallel() {
    thread::spawn(move || {
    //     // some work here
    //     create_api("task -a".to_string());
        create_api("ctlptl create cluster k3d --registry=ctlptl-registry".to_string());
        println!("Finished creating k3d cluster");

        // create_api("minikube create".to_string());
        // println!("Finished creating minikube cluster");


    });
    thread::spawn(move || {
        // some work here
        create_api("kind create cluster".to_string());
        println!("Finished creating kind cluster");
        // create_api("kind create cluster".to_string());
        println!("Finished creating kind cluster");
    });
}

#[tauri::command]
fn create_local_workspace(name: &str) -> String {
    install_clusters_parallel();
//     thread::spawn(move || {
//         create_api("task -a".to_string());
//         create_api("just start".to_string());
//     }};
//
// thread::spawn(move || {
// // some work here
// Command::new("sh")
// .arg("-c")
// .arg(command)
// .spawn()
// .expect("command failed to run");
// });

    // create_api("just list".to_string());
    create_api("ipconfig getifaddr en0".to_string());
    println!("new worl here {}", name);
    // command_run();
    format!("create_local_workspace {}", name);
    "sts".to_string()
    // todo!();
}

#[tauri::command]
fn delete_local_workspace(name: &str) -> String {
    format!("delete_local_workspace {}!", name);
    create_api("ctlptl delete cluster k3d".to_string());
    create_api("kind delete cluster".to_string());
    // thread::spawn( {
    //     // some work here
    //     Command::new("sh")
    //         .arg("-c")
    //         .arg("kind delete cluster")
    //         .spawn()
    //         .expect("command failed to run");
    // });
    "sts".to_string()
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
            delete_local_workspace
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
