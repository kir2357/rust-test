#[macro_use]
extern crate lazy_static;
extern crate toml;

// mod initialize{pub mod file_config;}
mod initialize;
use initialize::file_config::CONFIG;

fn main() {
    println!("aaaaaa");
    // 「as_str()」メソッドにて、文字型として呼び出しています。
    // 初めて「CONFIG」が参照されるタイミングで「file_config.rs」が実行され、設定ファイルの内容がメモリに展開されます。
    let char_code = CONFIG["csv"]["output"]["char_code"].as_str().unwrap();
    println!("char_code = {}", char_code);  // 標準出力結果「char_code = shift_jis」
    
    println!("bbbbbb");
    // 「as_integer()」メソッドにて、数値型として呼び出しています。
    // このタイミングでは、すでにメモリに展開された値を参照しているだけです。
    let value1 = CONFIG["calc"]["value1"].as_integer().unwrap();
    let value2 = CONFIG["calc"]["value2"].as_integer().unwrap();
    println!("value1 + value2 = {}", value1 + value2);  // 標準出力結果「value1 + value2 = 30」
}