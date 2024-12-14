use sill::*;

fn main() {
    app::message_box(
        &format!("🪟 {}", env!("CARGO_PKG_NAME")),
        &format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
    );
}
