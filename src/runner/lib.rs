use std::error::Error;
use std::fs;
use std::path::Path;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use crate::cli::CliArgs;

mod decompress;
mod progress;

pub fn run_upgrade(args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let mut sys = System::new_all();
    println!("等待海豹主进程关闭…");
    if args.pid != 0 {
        wait_exit_pid(args.pid, &mut sys);
    }

    if Path::new("./sealdice-core").exists() {
        fs::rename("./sealdice-core", "./sealdice-core_old")?;
    }

    decompress::decompress(&args.upgrade, "")?;
    println!("\r\x1b[32m解压成功!\x1b[0m{}", " ".repeat(7));

    Ok(())
}

fn wait_exit_pid(pid: u32, sys: &mut System) {
    let result = sys.process(Pid::from_u32(pid));
    if let Some(proc) = result {
        proc.wait();
    }
}