use chrono::{DateTime, Local, Datelike, Timelike};

#[derive(Eq, PartialEq)]
pub struct Troop(&'static str, u32);

impl Troop {
    pub fn colorized_name(&self) -> String {
        let s = format!("[{}m{}[0m", self.color(), self.name());
        return s;
    }
    pub fn name(&self) -> &'static str {
        self.0
    }
    pub fn color(&self) -> u32 {
        self.1
    }
}

const BEAST:   Troop = Troop("闇朱の獣牙兵団", 31);
const MACHINE: Troop = Troop("紫炎の鉄機兵団", 35);
const GOLEM:   Troop = Troop("深碧の造魔兵団", 32);
const ZONBIE:  Troop = Troop("蒼怨の屍獄兵団", 34);
const INSECT:  Troop = Troop("銀甲の凶蟲兵団", 33);
const RANDOM:  Troop = Troop("ランダム", 1);

// サイクル、日曜日の0時スタートでサイクルを定義、1時間毎の敵を記述
const CYCLE: [&Troop; 7] = [
    &MACHINE,
    &INSECT,
    &GOLEM,
    &ZONBIE,
    &INSECT,
    &RANDOM,
    &BEAST
];


pub fn calc_period(dt: DateTime<Local>) -> usize {
    (dt.weekday().num_days_from_sunday() * 24 + dt.hour()) as usize
}

pub fn get_troop_by_period(p: usize) -> &'static Troop {
    let index = p % CYCLE.len();
    CYCLE[index]
}
