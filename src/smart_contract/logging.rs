use log::{info, warn, error, debug};
use std::fs::OpenOptions;
use std::io::Write;

pub fn init_logging() {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("smart_contract.log")
        .unwrap();

    let _ = log::set_boxed_logger(Box::new(SimpleLogger { file: log_file }))
        .map(|()| log::set_max_level(log::LevelFilter::Info));
}

struct SimpleLogger {
    file: std::fs::File,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let log_entry = format!("{} - {}\n", record.level(), record.args());
            let _ = self.file.write_all(log_entry.as_bytes());
        }
    }

    fn flush(&self) {}
}
