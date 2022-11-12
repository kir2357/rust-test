#[macro_use]
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use std::panic::{self, PanicInfo};

fn setup_panic() {
    panic::set_hook(Box::new(move |panic_info: &PanicInfo<'_>| {
        let details = format!("{}", panic_info);
        println!("println:{}", details);
        error!("logger:{}", details);
    }));
}

fn setup_logger() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .encoder(Box::new(PatternEncoder::new(
            "{h({l})},{d},{f},{L},{M},{m},{n}",
        )))
        .build("log/output.log")
        // ファイル、フォルダの設定も行ってくれる
        // ファイルが存在していたら追記される
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
fn main() {
    setup_panic();
    setup_logger();

    debug!("debugです");
    info!("infoです");
    warn!("warnです");
    error!("errorです"); // 通常出力にはwarn以下は出ない

    println!("Hello, world!");

    panic!("test2");

    println!("after panic");
}
