
use chrono::{DateTime, Local};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;


pub fn setup_logger() {
    let now: DateTime<Local> = Local::now();
    let log_file_path = "log/".to_string() + &now.format("%Y-%m-%d_%H%M%S").to_string() + ".log";
    make_logger(&log_file_path);
}

#[cfg(debug_assertions)]
fn make_logger(log_file_path: &str) {
    // println!("デバッグ用のロガー");
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .encoder(Box::new(PatternEncoder::new(
            "{h({l})},{d},{f},{L},{M},{m},{n}",
        )))
        .build(log_file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Trace))
                // rust-analyzer でLevelFilterのリンク先で確認
        .unwrap();

    log4rs::init_config(config).unwrap();
}

#[cfg(not(debug_assertions))]
fn make_logger(log_file_path: &str) {
    // println!("リリース用のロガー");
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .encoder(Box::new(PatternEncoder::new(
            "{h({l})},{d},{f},{L},{M},{m},{n}",
        )))
        .build(log_file_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Warn))
        .unwrap();

    log4rs::init_config(config).unwrap();
}