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
    } else if let Some(path) = &args.exec_path {
        wait_exit_path(path, &mut sys);
    }

    depress::depress(&args.upgrade, "./rr")?;

    Ok(())
}

fn wait_exit_pid(pid: u32, sys: &mut System) {
    loop {
        let result = sys.process(Pid::from_u32(pid));
        if result.is_none() {
            break;
        }
        std::thread::sleep(Duration::from_secs(2));
        sys.refresh_all();
    }
}

fn wait_exit_path(path: impl AsRef<Path>, sys: &mut System) {
    loop {
        let mut found = false;
        for (_, proc) in sys.processes().iter() {
            let proc_path = proc.exe();
            if proc_path == path.as_ref() {
                found = true;
            }
        }
        if !found {
            break;
        }
        std::thread::sleep(Duration::from_secs(2));
        sys.refresh_all();
    }
}