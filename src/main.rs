use std::process::Command;
use std::path::Path;

use global::{CMD_OPT, SEAL_EXE};

use colorize::Colorize;
use log::{info, error};

use crate::logger::init_logger;

mod cli;
mod colorize;
mod global;
mod logger;
mod runner;

fn main() {
    let arg = &CMD_OPT;
    if arg.verbose {
        println!("{}", "Verbose mode turned on".yellow());
        println!("Working directory: {}", arg.cwd.yellow());
    }

    let ver = env!("CARGO_PKG_VERSION");
    println!(
        "{}",
        format!("SealDice 升级程序 v{} by 檀轶步棋", ver)
            .black()
            .on_yellow()
    );

    match init_logger() {
        Ok(name) => println!("本次升级日志将被写入 {}", name.yellow()),
        Err(e) => eprintln!("{}", format!("未能初始化升级日志: {}", e).red()),
    }

    info!("开始更新流程");
    if let Err(e) = runner::upgrade() {
        eprintln!("{}", format!("发生错误: {}", e).red());
        exit_gracefully(1);
    }

    info!("处理新文件并（如果可能）启动海豹");
    run_command(&arg.cwd);
}

fn exit_gracefully(code: i32) {
    if cfg!(windows) && code != 0 {
        use std::io::Read;
        println!("按回车键退出…");
        _ = std::io::stdin().read_exact(&mut [0u8]);
    }

    std::process::exit(code);
}

#[cfg(target_family = "unix")]
fn run_command(path: impl AsRef<Path>) {
    use std::os::unix::process::CommandExt;
    
    if CMD_OPT.verbose {
        println!(
            "Running `chmod` on {}",
            &path.as_ref().join(SEAL_EXE).to_string_lossy().on_yellow()
        );
        info!(
            "运行 `chmod` 于 {}",
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
                    eprintln!("From stderr: {}", err.on_red());
                    error!("`chmod` 返回的错误: {}", err);
                } else {
                    eprintln!("No error returned from stderr");
                }
            }
        }
        Err(err) => {
            eprintln!("\n{}\n", format!("出现错误: {}", err).red());
            error!("执行 `chmod` 出错: {}", err);
            exit_gracefully(1);
        }
    }

    if CMD_OPT.skip_startup {
        println!("{}", "Exiting due to flag --skip-startup".yellow());
        exit_gracefully(0);
    }

    println!("{}\n", "升级完毕，即将启动海豹核心…".black().on_yellow());

    std::thread::sleep(std::time::Duration::from_secs(2));
    let err = Command::new(Path::new("./").join(SEAL_EXE))
        .current_dir(path)
        .exec();
    eprintln!("\n{}\n", format!("出现错误: {}", err).red());
    error!("启动核心出错: {}", err);
    exit_gracefully(1);
}

#[cfg(target_family = "windows")]
fn run_command(path: impl AsRef<Path>) {
    if CMD_OPT.skip_startup {
        println!("{}", "Exiting due to flag --skip-startup".yellow());
        exit_gracefully(0);
    }

    println!("{}\n", "升级完毕，即将启动海豹核心…".black().on_yellow());

    std::thread::sleep(std::time::Duration::from_secs(2));
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
        eprintln!("\n{}\n", format!("出现错误: {}", err).red());
        error!("启动核心出错: {}", err);
        exit_gracefully(1);
    }
}
