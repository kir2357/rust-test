// env_loggerは標準出力には表示されるが > でテキストに出力できない
#[macro_use]
extern crate log;

use std::env;
use std::panic::{self, PanicInfo};

fn setup_panic() {
    panic::set_hook(Box::new(move |panic_info: &PanicInfo<'_>| {
        let details = format!("{}", panic_info);
        println!("{}", details);
    }));
}
fn main() {
    setup_panic();
    env::set_var("RUST_LOG", "info");
    // コンソールに出力するレベルの調整
    // 環境変数で調整
    env_logger::init();

    debug!("debugです");
    info!("infoです");
    warn!("warnです");
    error!("errorです"); // 通常出力にはwarn以下は出ない

    println!("Hello, world!");

    panic!("test2");

    println!("after panic")
}
