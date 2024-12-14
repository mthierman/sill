use sill::*;
use std::env;

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("xtask") => {
            app::message_box("🧰 xtask", "");
        }
        Some("rgb") => {
            let rgb = sill::macros::RGB(52, 64, 42);

            let r = sill::macros::GetRValue(rgb);
            let g = sill::macros::GetGValue(rgb);
            let b = sill::macros::GetBValue(rgb);

            println!("RGB:{:x} R:{} G:{} B:{}", rgb.0, r, g, b);
        }
        _ => {
            println!("No task!")
        }
    }
}
