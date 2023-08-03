use std::process;
use dq10tools::sinners;

fn main() {
    match sinners::get_current_levels() {
        Some(json) => println!("{}", json.to_string()),
        None => process::exit(1)
    }
}
