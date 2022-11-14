use confy;
use confy::ConfyError;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig { 
    version: i32, 
    address: String,
    port: i32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 2, 
            address: "localhost".to_string(),
            port: 3000
        }
    }
}
// ファイルが存在する⇒デフォルトをロード
// キー中途半端に存在　⇒　toml のパースエラー
fn main() -> Result<(), ConfyError> {
    let mut cfg = confy::load_path::<AppConfig>("config/confy_setting.toml")?;  // 設定ファイルの読込み
    cfg.version += 1;
    println!("{}",cfg.version);
    confy::store_path("config/confy_setting.toml", cfg)?;  // 設定ファイルの書込み
    Ok(())
}