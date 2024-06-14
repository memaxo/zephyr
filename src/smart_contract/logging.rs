use log::{info, warn, error, debug, LevelFilter};
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::sync::Mutex;
use simplelog::{Config, WriteLogger, CombinedLogger, LevelPadding, TerminalMode, ColorChoice, SimpleLogger as SimpleLog, WriteLogger as WriteLog};

pub fn init_logging(log_level: LevelFilter) {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("smart_contract.log")
        .unwrap();

    CombinedLogger::init(
        vec![
            SimpleLog::new(log_level, Config::default()),
            WriteLog::new(log_level, Config::default(), log_file),
        ]
    ).unwrap();
}

struct SimpleLogger {
    file: Mutex<File>,
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let log_entry = format!("{} - {}\n", record.level(), record.args());
            let mut file = self.file.lock().unwrap();
            let _ = file.write_all(log_entry.as_bytes());
            self.rotate_logs_if_needed(&mut file);
        }
    }

    fn flush(&self) {}

    fn rotate_logs_if_needed(&self, file: &mut File) {
        const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
        let metadata = file.metadata().unwrap();
        if metadata.len() > MAX_LOG_SIZE {
            let _ = file.set_len(0);
        }
    }
}
