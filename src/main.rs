use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{io, thread};
use std::io::Write;
use std::time::Duration;
use clap::Parser;
use crate::cli::CliArgs;

mod cli;
#[path= "runner/lib.rs"]
mod lib;

fn main() {
    let args = CliArgs::parse();
    if let Err(err) = lib::run_upgrade(&args) {
        println!("\n\x1b[31m出现错误：{}\x1b[0m", err);
        return;
    }

    println!("\x1b[33m海豹，启动！\x1b[0m\n");
    io::stdout().flush().unwrap();
    if !cfg!(windows) {
        Command::new("chmod").args(["+x", "./sealdice-core"]).spawn().unwrap();
        thread::sleep(Duration::from_secs(1));
        Command::new("./sealdice-core").exec();
    } else {
        Command::new("cmd")
            .args(["/C", "start", "", "sealdice-core.exe"])
            .exec();
    }
}
