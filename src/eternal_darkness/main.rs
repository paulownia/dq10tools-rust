use chrono::Local;
use std::process;

use dq10tools::eternal_darkness as enemy;

fn main() {
    match enemy::get_levels(Local::now()) {
        Some(json) => println!("{}", json.to_string()),
        None => process::exit(1),
    }
}

