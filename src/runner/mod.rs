use std::{error::Error, fs, path::Path};

use log::{info, warn};
use sysinfo::{Pid, PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

use crate::{
    colorize::Colorize,
    global::{CMD_OPT, SEAL_EXE},
};

mod decompress;
mod progress;

pub fn upgrade() -> Result<(), Box<dyn Error>> {
    let args = &CMD_OPT;
    if args.pid != 0 {
        println!("等待海豹主进程关闭...");
        info!("等待 PID: {}", args.pid);
        wait_proc(args.pid);
    }

    let seal_path = Path::new(&args.cwd);
    if seal_path.join(SEAL_EXE).exists() {
        #[cfg(target_family = "windows")]
        let old_name = format!("{}.old", SEAL_EXE);
        #[cfg(target_family = "unix")]
        let old_name = format!("{}_old", SEAL_EXE);
        fs::rename(seal_path.join(SEAL_EXE), seal_path.join(old_name)).map_err(|e| {
            warn!("未能备份旧文件: {}", e);
            e
        })?;
    }

    decompress::decompress(&args.upgrade, &args.cwd)?;
    println!("\r{}", "解压成功!".green());
    info!("成功解压 {}", args.upgrade);

    Ok(())
}

fn wait_proc(pid: u32) {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );

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
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

