fn main() {
    logger::panic::set_panic_hook();
    panic!("Test message");
}
