// ./logging_example > output.txt で標準出力がtxtに記録
// パニックハンドラでパニック情報も得られる

use std::panic::{self, PanicInfo};

fn setup_panic() {
    panic::set_hook(Box::new(move |panic_info: &PanicInfo<'_>| {
        let details = format!("{}", panic_info);
        println!("{}", details);
    }));
}
fn main() {
    setup_panic();
    println!("Hello, world!");

    panic!("aaaaaa");

    println!("after panic")
}
