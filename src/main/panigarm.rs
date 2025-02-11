use dq10tools::panigarm;

fn main() {
    let cycle = panigarm::get_next_cycle();

    for event in cycle {
        println!("{} {}", event.started_at.format("%m/%d %H:%M"), bold(event.boss.name));
    }
}

fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}
