use std::process;
use dq10tools::sinners;
use serde_json::Value;
use chrono::prelude::*;
use chrono_tz::Asia::Tokyo;
use anyhow::{Context, Result};

fn main() {
    match sinners::get_current_levels() {
        Some(j) => if let Err(e) = print_json(j) {
            eprintln!("{}", e);
            process::exit(1);
        },
        None => {
            eprintln!("no data");
            process::exit(1)
        }
    }
}

fn to_jst_str(o: Option<&str>) -> Result<String> {
    let s = o.context("no term")?;
    let dt = DateTime::parse_from_rfc3339(s)?;
    let s = dt.with_timezone(&Tokyo).format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(s)
}

fn print_json(j: Value) -> Result<()> {
    let from = to_jst_str(j["term"]["from"].as_str())?;
    let to = to_jst_str(j["term"]["to"].as_str())?;
    let level = j["level"].as_object().context("no level")?;

    println!("{} から {} まで", from, to);
    for (k, v) in level {
        println!("{}\t{}", k, v);
    }

    Ok(())
}
