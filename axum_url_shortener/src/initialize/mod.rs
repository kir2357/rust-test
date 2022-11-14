use std::panic::{self, PanicInfo};
mod setup_logger;

mod load_config;
pub use load_config::CONFIG;

pub fn init() {
    setup_panic();
    setup_logger::setup_logger();
}

fn setup_panic() {
    panic::set_hook(Box::new(move |panic_info: &PanicInfo<'_>| {
        let details = format!("{}", panic_info);
        // println!("println:{}", details);
        error!("{}", details);
    }));
}