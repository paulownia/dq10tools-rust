use chrono::{Local};
use std::process;

use dq10tools::saint_guardians::enemy::*;

fn main() {
    match get_enemy_levels(Local::now()) {
        Some(json) => println!("{}", json.to_string()),
        None => process::exit(1),
    }
}

