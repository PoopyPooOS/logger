use logger::{Colorize, Location, location::Section, warn};

fn main() {
    logger::panic::set_panic_hook();
    let location = Location::from_path("examples/highlighted/bad_code.rs")
        .expect("Failed to read file")
        .section(Section::new(1..=1, 20..=22));

    warn!(
        location: location,
        hint: format!("Remove {} in {} call", "'\\n'".bold(), "'println!'".bold()),
        "Using new line in println! call",
    );
}
