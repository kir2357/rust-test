use confy;
// use confy::ConfyError;
use serde_derive::{Deserialize, Serialize};
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<AppConfig> = Lazy::new(||set_config());

const CONFIG_PATH: &str = "config/confy_setting.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig { 
    pub version: i32, 
    pub address: String,
    pub port: i32,
}
// フィールドもpub にしないとアクセスできない

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 2, 
            address: "localhost".to_string(),
            port: 3000
        }
    }
}
fn set_config() -> AppConfig {
    match confy::load_path::<AppConfig>(CONFIG_PATH){
        Ok(v) => v,
        Err(_e) => AppConfig::default(),
    }
}
