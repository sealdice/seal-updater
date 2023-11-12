use crate::global::{CMD_OPT, SEAL_EXE};
use colored::Colorize;
use std::io::{stdin, Read, Write};
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::{io, process};

mod cli;
mod global;
#[path = "runner/lib.rs"]
mod lib;

fn main() {
    let args = &CMD_OPT;
    if args.verbose {
        println!("{}", "VERBOSE mode on".yellow());
        println!("工作路径已被设定为: {}", args.cwd.yellow())
    }
    println!("{}", "SealDice 升级程序 by 檀轶步棋".black().on_yellow());

    if let Err(err) = lib::run_upgrade() {
        println!("\n{}\n", format!("出现错误: {}", err).red());
        exit_gracefully(1);
    }

    io::stdout().flush().unwrap();
    run_command(&args.cwd);
}

#[cfg(target_family = "unix")]
fn run_command(path: impl AsRef<Path>) {
    use std::os::unix::process::CommandExt;

    if CMD_OPT.verbose {
        println!(
            "Running `chmod` on {}",
            &path.as_ref().join(SEAL_EXE).to_string_lossy().on_yellow()
        );
    }
    let res = Command::new("chmod")
        .args(["+x", &path.as_ref().join(SEAL_EXE).to_string_lossy()])
        .output();
    match res {
        Ok(o) => {
            if CMD_OPT.verbose {
                let err = o.stderr;
                if err.len() > 0 {
                    let err = String::from_utf8(err).unwrap_or_default();
                    println!("from stderr: {}", err.on_red());
                } else {
                    println!("no error returned from stderr");
                }
            }
        }
        Err(err) => {
            println!("\n{}\n", format!("出现错误: {}", err).red());
            exit_gracefully(1);
        }
    }

    if CMD_OPT.skip_startup {
        println!("{}", "Exiting due to flag --skip-startup".yellow());
        exit_gracefully(0);
    }

    println!("{}\n", "升级完毕，即将启动海豹核心…".black().on_yellow());

    thread::sleep(Duration::from_secs(2));
    let err = Command::new(Path::new("./").join(SEAL_EXE))
        .current_dir(path)
        .exec();
    println!("\n{}\n", format!("出现错误: {}", err).red());
    exit_gracefully(1);
}

#[cfg(target_family = "windows")]
fn run_command(path: impl AsRef<Path>) {
    if CMD_OPT.skip_startup {
        println!("{}", "Exiting due to flag --skip-startup".yellow());
        exit_gracefully(0);
    }

    println!("{}\n", "升级完毕，即将启动海豹核心…".black().on_yellow());

    thread::sleep(Duration::from_secs(2));
    if let Err(err) = Command::new("cmd")
        .current_dir(path)
        .args([
            "/C",
            "start",
            "",
            &Path::new("./").join(SEAL_EXE).to_string_lossy(),
        ])
        .spawn()
    {
        println!("\n{}\n", format!("出现错误: {}", err).red());
        exit_gracefully(1);
    }
}

fn exit_gracefully(code: i32) {
    if cfg!(windows) && code != 0 {
        println!("按回车键退出…");
        _ = stdin().read_exact(&mut [0u8]);
    }

    process::exit(code);
}
