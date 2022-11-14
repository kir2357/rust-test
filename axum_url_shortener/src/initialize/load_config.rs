use confy;
use serde_derive::{Deserialize, Serialize};
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<AppConfig> = Lazy::new(||set_config());

const CONFIG_PATH: &str = "config/axum_url_shortener.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig { 
    pub version: i32, 
    pub url: String,
}
// フィールドもpub にしないとアクセスできない

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            version: 1, 
            url: "127.0.0.1:4000".to_string(),
        }
    }
}
fn set_config() -> AppConfig {
    match confy::load_path::<AppConfig>(CONFIG_PATH){
        Ok(v) => v,
        Err(_e) => AppConfig::default(),
    }
}
