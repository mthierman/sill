use sill::*;

fn main() {
    message_box(
        &format!("🪟 {}", env!("CARGO_PKG_NAME")),
        &format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
    );
}
