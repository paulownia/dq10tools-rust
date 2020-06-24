use chrono::{Local};
use std::process;

use dq10tools::saint_guardians as enemy;

fn main() {
    match enemy::get_levels(Local::now()) {
        Some(json) => println!("{}", json.to_string()),
        None => process::exit(1),
    }
}

