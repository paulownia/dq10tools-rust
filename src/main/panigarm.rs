use dq10tools::panigarm;

fn main() {
    let boss = panigarm::get_current_boss();
    let next_boss = panigarm::get_next_boss();
    let next_change_time = panigarm::get_next_change_time(chrono::Local::now());

    // ボス名称はターミナルで太字表示にする
    println!("現在のボスは{}です", bold(&boss.name));
    println!("{}から{}になります", bold(next_change_time.format("%-m月%-d日%-H時").to_string().as_str()), bold(&next_boss.name));
}

fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}
