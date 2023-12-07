use std::path::Path;
use std::process::Command;

use global::{CMD_OPT, SEAL_EXE};

use colorize::Colorize;
use log::{error, info};

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

    match init_logger(CMD_OPT.no_log) {
        Ok(name) => {
            if name != "" {
                println!("本次升级日志将被写入 {}", name.yellow())
            }
        }
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
    use std::{fs, os::unix::fs::PermissionsExt, os::unix::process::CommandExt};

    // Do not use fs::metadata and then metadata::set_mode, because it does nothing.
    let perm_res = fs::set_permissions(
        &path.as_ref().join(SEAL_EXE),
        PermissionsExt::from_mode(0o755),
    );
    match perm_res {
        Ok(_) => {
            info!("将 {:?} 权限设置为 0755", &path.as_ref().join(SEAL_EXE));
        }
        Err(e) => {
            error!("设置 {:?} 权限时出错: {}", &path.as_ref().join(SEAL_EXE), e);
            eprintln!("设置 sealdice-core 权限时出错: {}", e);
            exit_gracefully(1);
        }
    }

    if CMD_OPT.skip_startup {
        println!("{}", "因 --skip-startup，现在退出程序".yellow());
        info!("因 --skip-startup，现在退出程序");
        exit_gracefully(0);
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{}\n", "升级完毕，即将启动海豹核心…".black().on_yellow());
    info!(
        "准备运行海豹主程序。如果海豹没有启动，但下面没有出现报错信息，应该是 {} 的问题",
        SEAL_EXE
    );

    std::thread::sleep(std::time::Duration::from_secs(2));
    let err = Command::new(path.as_ref().join(SEAL_EXE))
        .current_dir(path)
        .exec();
    eprintln!("\n{}\n", format!("出现错误: {}", err).red());
    error!("启动核心出错: {}", err);
    exit_gracefully(1);
}

#[cfg(target_family = "windows")]
fn run_command(path: impl AsRef<Path>) {
    if CMD_OPT.skip_startup {
        println!("{}", "因 --skip-startup，现在退出程序".yellow());
        info!("因 --skip-startup，现在退出程序");
        exit_gracefully(0);
    }

    println!("{}\n", "升级完毕，即将启动海豹核心…".black().on_yellow());
    info!(
        "准备运行海豹主程序。如果海豹没有启动，但下面没有出现报错信息，应该是 {} 的问题",
        SEAL_EXE
    );

    std::thread::sleep(std::time::Duration::from_secs(2));
    if let Err(err) = Command::new("cmd")
        .current_dir(path)
        .args(["/C", "start", "", path.as_ref().join(SEAL_EXE)])
        .spawn()
    {
        eprintln!("\n{}\n", format!("出现错误: {}", err).red());
        error!("启动核心出错: {}", err);
        exit_gracefully(1);
    }
}
