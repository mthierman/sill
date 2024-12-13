use sill::*;
use std::env;

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("xtask") => {
            message_box("🧰 xtask", "");
        }
        _ => {
            println!("No task!")
        }
    }
}
