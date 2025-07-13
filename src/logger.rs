use log::{Level, Metadata, Record};

pub static LOGGER: Logger = Logger{log_level: Level::Debug};

pub struct Logger {
    log_level: Level,
}

pub fn init(level: Level) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(level.to_level_filter());
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.log_level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
