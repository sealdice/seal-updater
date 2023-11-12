use crate::global::{CMD_OPT, SEAL_EXE};
use colored::Colorize;
use std::error::Error;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};
use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

mod decompress;
mod progress;

pub fn run_upgrade() -> Result<(), Box<dyn Error>> {
    let args = &CMD_OPT;
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );
    if args.pid != 0 {
        println!("等待海豹主进程关闭…");
        wait_exit_pid(args.pid, &mut sys);
    }

    let seal_path = Path::new(&args.cwd);
    if seal_path.join(SEAL_EXE).exists() {
        #[cfg(target_family = "windows")]
        let old_name = format!("{}.old", SEAL_EXE);
        #[cfg(target_family = "unix")]
        let old_name = format!("{}_old", SEAL_EXE);
        fs::rename(seal_path.join(SEAL_EXE), seal_path.join(old_name))?;
    }

    decompress::decompress(&args.upgrade, &args.cwd)?;
    println!("\r{}", "解压成功!".green());

    Ok(())
}

fn wait_exit_pid(pid: u32, sys: &mut System) {
    loop {
        let result = sys.process(Pid::from_u32(pid));
        if let Some(proc) = result {
            if proc.name() == "seal-updater" {
                break;
            }
        } else {
            break;
        }
        sys.refresh_processes();
        thread::sleep(Duration::from_secs(1));
    }
}
