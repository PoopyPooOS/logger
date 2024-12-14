use log::info;

fn main() {
    logger::init().expect("Failed to set logger");
    info!("Hello, world!");
}
