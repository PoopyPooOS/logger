use logger::{warn, Colorize, Location};

fn main() {
    logger::panic::set_panic_hook();
    let location = Location::from_path("examples/highlighted/bad_code.rs", 1..=1)
        .expect("Failed to read file")
        .section(21..=22);

    warn!(
        "Using new line in println! call",
        location: location,
        hint: format!("Remove {} in {} call", "\\n".bold(), "println!".bold())
    );
}
