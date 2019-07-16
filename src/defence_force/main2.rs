use dq10tools::defence_force;

fn main() {
    let schedule = defence_force::schedule::get_current_schedule();

    for event in schedule {
        let time_str = event.started_at.format("%m/%d %H:%M");
        println!("{} {}", time_str, event.troop.colorized_name());
    }
}

