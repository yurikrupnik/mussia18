// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// enum LocalClusterEnum {
//     KIND,
//     MINIKUBE,
//     K3D
// }
//
// enum MachineTypes {
//     mobile,
//     tablet_small,
//     tablet_large,
//     desktop_small,
//     desktop_medium,
//     desktop_large,
// }
//
// enum MonorepoEnum {
//     NX,
//     LERNA,
//     TURBOREPO,
//     YARN,
//     BAZEL,
//     NONE
// }
//
// enum CloudProviders {
//     Google,
//     Amazon,
//     TURBOREPO,
//     YARN,
//     BAZEL,
//     NONE
// }
// struct User {
//     email: String,
//     age: i8,
//     id: String
// }
//
// struct Chart {}

// struct Project {
//     name: String,
//     cloud: CloudProviders,
//     machine: MachineTypes,
//     helm_charts: Vec<Chart>
// }
//
// #[derive(Debug, Default)]
// struct LocalCluster {
//     monorepo: MonorepoEnum,
//     kind: LocalClusterEnum,
//     machine_type: MachineTypes,
//     cpu: i16,
//     momery: u8,
//     count: i16,
//     user: User,
//     user_id: String,
//     commands: Vec<String>,
//     project: Project
// }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_local_cluster() {
    // std::thread::spawn:
    todo!()
}

#[tauri::command]
fn delete_local_cluster() {
    todo!()
    // std::thread::spawn:

}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, create_local_cluster, delete_local_cluster])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
