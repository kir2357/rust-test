use config::ConfigError;
use dotenv;
use serde::Deserialize;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(||set_config());

#[derive(Deserialize, Debug)]
pub struct Config {
    pub address: String,
    pub port: i32,
}

impl Config {
    /// 環境変数からデータを読み込む
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }

    // pub fn use_default() -> Self{

    // }
}

fn set_config() -> Config{
    println!("1");
    dotenv::from_filename("config/dotenv_example.env").ok();
    println!("2");
    Config::from_env().unwrap() 
    // match  Config::from_env() {
    //      => 
    // }
}
