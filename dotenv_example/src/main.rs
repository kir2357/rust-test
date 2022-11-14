mod initialize;
use initialize::load_config::CONFIG;

fn main() {
    println!("address: {}", CONFIG.address);
    println!("port: {}", CONFIG.port);
}
