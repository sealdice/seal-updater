use crate::global::CMD_OPT;

/// Initialize the logger, making it create a new file and write to that file.
/// If `no_log` in [CMD_OPT] is set to `true` with `--no-log` flag,
/// all logging methods will be no-ops.
pub fn init_logger() -> Result<String, fern::InitError> {
    let mut file_name = String::new();
    let log_level = if CMD_OPT.no_log {
        log::LevelFilter::Off
    } else {
        log::LevelFilter::Info
    };

    let cfg = fern::Dispatch::new()
        .format(|out, msg, rec| {
            out.finish(format_args!(
                "{} [{}] {}:{} {}",
                chrono::Local::now().format("%F %H:%M:%S%.3f"),
                rec.level(),
                rec.file().unwrap_or("unknown"),
                rec.line().unwrap_or(0),
                msg
            ))
        })
        .level(log_level);

    if !CMD_OPT.no_log {
        let date = chrono::Local::now().format("%F_%H%M%S").to_string();
        file_name = format!("升级日志_{}.txt", date);

        cfg.chain(fern::log_file(&file_name)?).apply()?;
    } else {
        cfg.apply()?;
    }

    Ok(file_name)
}
