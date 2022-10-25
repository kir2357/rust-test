#![warn(dead_code)]

#[derive(Default)]
pub struct Second {
    value: u64
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}


#[test]
fn main () {
    println!("second:{:?}",Second::default().value() );
}