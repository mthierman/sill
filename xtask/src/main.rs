use sill::*;
use std::env;

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("xtask") => {
            message_box("🧰 xtask", "");
        }
        Some("rgb") => {
            let rgb = RGB(52, 64, 42);

            let r = GetRValue(rgb);
            let g = GetGValue(rgb);
            let b = GetBValue(rgb);

            println!("RGB:{:x} R:{} G:{} B:{}", rgb.0, r, g, b);
        }
        _ => {
            println!("No task!")
        }
    }
}
