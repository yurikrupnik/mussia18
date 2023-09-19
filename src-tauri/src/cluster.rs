use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::thread;

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
pub fn create_api(command: String) -> std::io::Result<()> {
    thread::spawn(move || {
        // some work here
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("command failed to run");
    });
    Ok(())
}
