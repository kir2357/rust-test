// extern crate log4rs;
// extern crate chrono;

// use chrono::{DateTime, Utc};
use chrono::{DateTime, Local};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use std::panic::{self, PanicInfo};

pub fn init() {
    setup_panic();

    // let log_file_path = "log/test_output.log";
    // let now: DateTime<Utc> = Utc::now();
    let now: DateTime<Local> = Local::now();

    // println!("UTC now is: {}", now);
    // println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    // println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    // println!(
    //     "UTC now in a custom format is: {}",
    //     now.format("%a %b %e %T %Y")
    // );
    // println!("test:{}", now.format("%Y-%m-%d_%H%M%S"));

    let log_file_path = "log/".to_string() + &now.format("%Y-%m-%d_%H%M%S").to_string() + ".log";

    setup_logger(&log_file_path);
}

fn setup_panic() {
    panic::set_hook(Box::new(move |panic_info: &PanicInfo<'_>| {
        let details = format!("{}", panic_info);
        println!("println:{}", details);
        error!("logger:{}", details);
    }));
}

fn setup_logger(log_file_path: &str) {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .encoder(Box::new(PatternEncoder::new(
            "{h({l})},{d},{f},{L},{M},{m},{n}",
        )))
        .build(log_file_path)
        // ファイル、フォルダの設定も行ってくれる
        // ファイルが存在していたら追記される
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
