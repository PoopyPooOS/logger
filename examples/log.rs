#[cfg(feature = "log")]
fn main() {
    logger::init().expect("Failed to set logger");
    log::info!("Hello, world!");
}

#[cfg(not(feature = "log"))]
fn main() {
    logger::error!("Enable the `log` feature to use this example");
}
