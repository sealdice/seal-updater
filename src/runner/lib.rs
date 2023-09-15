use std::error::Error;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use crate::cli::CliArgs;

mod decompress;
mod progress;

pub fn run_upgrade(args: &CliArgs) -> Result<(), Box<dyn Error>> {
    let mut sys = System::new_all();
    println!("等待海豹主进程关闭……");
    if let Some(pid) = args.pid {
        wait_exit_pid(pid, &mut sys);
    }
    decompress::decompress(&args.upgrade, "./rr")?;

    Ok(())
}

fn wait_exit_pid(pid: u32, sys: &mut System) {
    let result = sys.process(Pid::from_u32(pid));
    if let Some(proc) = result {
        proc.wait();
    }
}