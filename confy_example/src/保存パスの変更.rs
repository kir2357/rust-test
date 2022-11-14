use confy;
use confy::ConfyError;
use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
struct AppConfig { version: i32, }

fn main() -> Result<(), ConfyError> {
    let mut cfg = confy::load_path::<AppConfig>("config/confy_setting.toml")?;  // 設定ファイルの読込み
    cfg.version += 1;
    println!("{}",cfg.version);
    confy::store_path("config/confy_setting.toml", cfg)?;  // 設定ファイルの書込み
    Ok(())
}