use chrono::{Timelike};
use dq10tools::ast;

fn main() {
    let astortia_time = ast::now();
    let naive_time = astortia_time.time();
    println!("現在のアストルティア時刻は{: >02}時{: >02}分{: >02}秒", naive_time.hour(), naive_time.minute(), naive_time.second());

    println!("約{}分で{}になります", astortia_time.state_change_in(), astortia_time.state().change())
}
