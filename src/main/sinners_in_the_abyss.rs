use chrono::{Local};
use std::process;
use dq10tools::sinners;

fn main() {
    match sinners::get_levels(Local::now()) {
        Some(json) => println!("{}", json.to_string()),
        None => process::exit(1)
    }
}
