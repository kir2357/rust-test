use dotenv;

pub fn initialize() {
    env_logger::init();
    dotenv::from_filename("config/logger_setting.env").ok();
    // ここで設定ファイルがないことは分からない
    // dotenv::dotenv().ok();
    // env::set_var("RUST_LOG", "info");
}