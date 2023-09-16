use std::error::Error;
use std::{fs, thread};
use std::path::Path;
use std::time::Duration;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use crate::cli::CliArgs;
use crate::SEAL_EXE;

mod decompress;
mod progress;

pub fn run_upgrade(args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let mut sys = System::new_all();
    println!("等待海豹主进程关闭…");
    if args.pid != 0 {
        wait_exit_pid(args.pid, &mut sys);
    }

    if Path::new(SEAL_EXE).exists() {
        #[cfg(target_family = "windows")]
        let old_name = format!("{}.old", SEAL_EXE);
        #[cfg(target_family = "unix")]
        let old_name = format!("{}_old", SEAL_EXE);
        fs::rename(SEAL_EXE, old_name)?;
    }

    decompress::decompress(&args.upgrade, "")?;
    println!("\r\x1b[32m解压成功!\x1b[0m{}", " ".repeat(7));

    Ok(())
}

fn wait_exit_pid(pid: u32, sys: &mut System) {
    loop {
        let result = sys.process(Pid::from_u32(pid));
        if result.is_none() {
            break;
        }
        sys.refresh_processes();
        thread::sleep(Duration::from_secs(1));
    }
}

fn _wait_exit_pid(pid: u32, sys: &mut System) {
    let result = sys.process(Pid::from_u32(pid));
    if let Some(proc) = result {
        proc.wait();
    }
}