use crate::cli::CliArgs;
use clap::Parser;
use std::io;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;

mod cli;
#[path = "runner/lib.rs"]
mod lib;

#[cfg(target_family = "windows")]
const SEAL_EXE: &str = "sealdice-core.exe";
#[cfg(target_family = "unix")]
const SEAL_EXE: &str = "./sealdice-core";

fn main() {
    let args = CliArgs::parse();
    if let Err(err) = lib::run_upgrade(&args) {
        println!("\n\x1b[31m出现错误：{}\x1b[0m", err);
        return;
    }

    if args.debug {
        return;
    }

    println!("\x1b[33m海豹，启动！\x1b[0m\n");
    io::stdout().flush().unwrap();
    run_command();
}

#[cfg(target_family = "unix")]
fn run_command() {
    use std::os::unix::process::CommandExt;
    if let Err(e) = Command::new("chmod").args(["+x", SEAL_EXE]).spawn() {
        println!("\x1b[31m启动失败：{}\x1b[0m\n", e);
    }
    thread::sleep(Duration::from_secs(1));
    let err = Command::new(SEAL_EXE).exec();
    println!("\x1b[31m启动失败：{}\x1b[0m\n", err);
}

#[cfg(target_family = "windows")]
fn run_command() {
    thread::sleep(Duration::from_secs(2));
    if let Err(e) = Command::new("cmd")
        .args(["/C", "start", "", SEAL_EXE])
        .spawn()
    {
        println!("\x1b[31m启动失败：{}\x1b[0m\n", e);
    }
}
