use dq10tools::defence_force;
use chrono::{Local, DateTime, TimeZone};
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

fn main() {
    let args = Args::from_args();

    let dt: DateTime<Local> = match args.dt {
        None => Local::now(),
        Some(dt_str) => match Local.datetime_from_str(dt_str.as_str(), "%Y-%m-%d %H:%M") {
            Ok(dt) => dt,
            Err(_) => {
                println!("invalid date {}", dt_str);
                process::exit(1)
            }
        },
    };

    let opt_schedule = defence_force::schedule::get_schedule_in(dt, args.count);

    if let Some(schedule) = opt_schedule {
        for event in schedule {
            let time_str = event.started_at.format("%m/%d %H:%M");
            println!("{} {}", time_str, event.troop.colorized_name());
        }
    }
}

