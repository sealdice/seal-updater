pub fn init_logger() -> Result<String, fern::InitError> {
    let time = chrono::Local::now().format("%F_%H%M%S").to_string();
    let file_name = format!("升级日志_{}.txt", time);

    fern::Dispatch::new()
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
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(&file_name)?)
        .apply()?;

    Ok(file_name)
}
