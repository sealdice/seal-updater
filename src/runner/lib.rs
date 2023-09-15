use std::error::Error;
use std::path::Path;
use std::time::Duration;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use crate::cli::CliArgs;

mod depress;
mod progress;

pub fn run_upgrade(args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let mut sys = System::new_all();
    println!("等待海豹主进程关闭……");
    if let Some(pid) = args.pid {
        wait_exit_pid(pid, &mut sys);
    }
    depress::depress(&args.upgrade, "./rr")?;

    Ok(())
}

fn wait_exit_pid(pid: u32, sys: &mut System) {
    let result = sys.process(Pid::from_u32(pid));
    if let Some(proc) = result {
        proc.wait();
    }
}