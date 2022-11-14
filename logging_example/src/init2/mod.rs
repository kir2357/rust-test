use std::panic::{self, PanicInfo};
mod logger;

pub fn init() {
    setup_panic();
    logger::setup_logger();
}

fn setup_panic() {
    panic::set_hook(Box::new(move |panic_info: &PanicInfo<'_>| {
        let details = format!("{}", panic_info);
        println!("println:{}", details);
        error!("logger:{}", details);
    }));
}