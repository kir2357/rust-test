#[macro_use]
extern crate log;
// extern crateはrootファイルにしか書けない

mod init;

fn main() {
    init::init();

    debug!("debugです");
    info!("infoです");
    warn!("warnです");
    error!("errorです"); // 通常出力にはwarn以下は出ない

    println!("Hello, world!");

    panic!("test2");

    println!("after panic");
}
