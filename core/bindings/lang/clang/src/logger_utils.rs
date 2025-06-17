use nogamepads::logger_utils::logger_build;

#[unsafe(no_mangle)]
pub extern "C" fn enable_logger(level: u8) {
    let level_filter = match level {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        2 => log::LevelFilter::Trace,
        _ => { log::LevelFilter::Info }
    };

    logger_build(level_filter);
}