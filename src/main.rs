use chrono::{DateTime, Local};
use std::mem;

fn main() {
    print!("{}", mem::size_of::<DateTime<Local>>())
}
