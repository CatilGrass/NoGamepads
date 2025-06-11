use env_logger::Builder;
use log::LevelFilter;

pub fn logger_build(level: LevelFilter) {
    Builder::new()
        .format(|buf, record| {
            use std::io::Write;
            let now = chrono::Local::now();
            let level_style = buf.default_level_style(record.level());
            writeln!(
                buf,
                "[{}] [{}] {}",
                now.format("%H:%M:%S"),
                level_style.value(record.level()),
                record.args()
            )
        })
        .filter(None, level)
        .init();
}