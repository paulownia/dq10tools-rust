use dq10tools::defence_force;
use chrono::{Local, DateTime, TimeZone, NaiveDateTime};
use structopt::StructOpt;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(name = "Defence Force Schedule")]
struct Args {
    #[structopt(short = "d")]
    dt: Option<String>,


    #[structopt(short = "c", default_value="24")]
    count: usize,
}

fn parse_as_local_datetime(s: &str) -> Result<DateTime<Local>, String>  {
    match NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M") {
        Ok(dt) => match Local.from_local_datetime(&dt).single() {
            Some(dt) => Ok(dt),
            None => Err(format!("ambiguous date  {}.", s))
        },
        Err(e) => Err(format!("invalid date format {}. {}.", s, e))
    }
}

fn main() {
    let args = Args::from_args();

    let dt: DateTime<Local> = match args.dt {
        None => Local::now(),
        Some(dt_str) => match parse_as_local_datetime(&dt_str) {
            Ok(dt) => dt,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        }
    };

    let opt_schedule = defence_force::schedule::get_schedule_in(dt, args.count);

    if let Some(schedule) = opt_schedule {
        for event in schedule {
            let time_str = event.started_at.format("%m/%d %H:%M");
            println!("{} {}", time_str, event.troop.colorized_name());
        }
    }
}

