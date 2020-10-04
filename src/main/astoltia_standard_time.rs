use chrono::{Timelike};
use dq10tools::ast;

fn main() {
    let nt = ast::now();
    let next = ast::calc_minutes_to_next(nt);
    println!("現在のアストルティア時刻は{: >02}時{: >02}分{: >02}秒", nt.hour(), nt.minute(), nt.second());
    println!("約{}分で{}になります", next.after_minutes, next.state)

}
