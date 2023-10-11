use crate::cli::CliArgs;
use clap::Parser;
use std::io::{stdin, Read, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::{io, process};

mod cli;
#[path = "runner/lib.rs"]
mod lib;

#[cfg(target_family = "windows")]
const SEAL_EXE: &str = "sealdice-core.exe";
#[cfg(target_family = "unix")]
const SEAL_EXE: &str = "sealdice-core";

fn main() {
    let args = CliArgs::parse();
    println!("\x1b[43m\x1b[30mSealDice 升级程序 by 檀轶步棋\x1b[0m");

    if let Err(err) = lib::run_upgrade(&args) {
        println!("\n\x1b[31m出现错误：{}\x1b[0m\n", err);
        exit_gracefully(1);
    }

    if args.debug {
        exit_gracefully(0);
    }

    println!("\x1b[43m\x1b[30m升级完毕，即将启动海豹核心…\x1b[0m\n");
    io::stdout().flush().unwrap();
    run_command(Path::new(&args.cwd));
}

#[cfg(target_family = "unix")]
fn run_command(path: impl AsRef<Path>) {
    use std::os::unix::process::CommandExt;
    if let Err(e) = Command::new("chmod")
        .args(["+x", &path.as_ref().join(SEAL_EXE).to_string_lossy()])
        .spawn()
    {
        println!("\x1b[31mchmod 失败：{}\x1b[0m\n", e);
        exit_gracefully(1);
    }
    thread::sleep(Duration::from_secs(2));
    let err = Command::new(Path::new("./").join(SEAL_EXE))
        .current_dir(path)
        .exec();
    println!("\x1b[31m启动失败：{}\x1b[0m\n", err);
    exit_gracefully(1);
}

#[cfg(target_family = "windows")]
fn run_command(path: impl AsRef<Path>) {
    thread::sleep(Duration::from_secs(2));
    if let Err(e) = Command::new("cmd")
        .current_dir(path)
        .args([
            "/C",
            "start",
            "",
            &Path::new("./").join(SEAL_EXE).to_string_lossy(),
        ])
        .spawn()
    {
        println!("\x1b[31m启动失败：{}\x1b[0m\n", e);
        exit_gracefully(1);
    }
}

fn exit_gracefully(code: i32) {
    if cfg!(windows) && code != 0 {
        println!("按任意键退出…");
        _ = stdin().read(&mut [0u8]);
    }

    process::exit(code);
}
