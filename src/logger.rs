/// Initialize the logger, making it create a new file and write to that file.
/// If `no_log` in [CMD_OPT] is set to `true` with `--no-log` flag,
/// all logging methods will be no-ops.
pub fn init_logger(no_log: bool) -> Result<String, fern::InitError> {
    let (log_level, file_name) = if no_log {
        (log::LevelFilter::Off, String::new())
    } else {
        let date = chrono::Local::now().format("%F_%H%M%S").to_string();
        (log::LevelFilter::Debug, format!("upgrade_log_{}.txt", date))
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

    if !no_log {
        cfg.chain(fern::log_file(&file_name)?).apply()?;
    } else {
        cfg.apply()?;
    }

    Ok(file_name)
}
