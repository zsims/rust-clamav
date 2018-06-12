extern crate clamav;

pub fn setup() {
    clamav::initialize().expect("initialize to succeed");
}
