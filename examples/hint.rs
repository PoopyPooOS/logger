use logger::fatal;

fn main() {
    fatal!(
        hint: "Please make sure config.toml exists",
        "Failed to locate config file",
    );
}
