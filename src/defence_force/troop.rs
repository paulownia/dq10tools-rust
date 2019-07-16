use chrono::{TimeZone, DateTime, Local, Duration, Datelike, Timelike};

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




pub struct Event {
    pub started_at: DateTime<Local>,
    pub troop: &'static Troop
}

pub fn get_current_schedule() -> Vec<Event> {
    get_schedule(Local::now())
}

pub fn get_schedule(dt: DateTime<Local>) -> Vec<Event> {
    let mut vec: Vec<Event> = Vec::with_capacity(24);
    let period = calc_period(dt);

    let dt = Local.ymd(dt.year(), dt.month(), dt.day()).and_hms(dt.hour(), 0, 0);

    for i in 0..24 {
        let x = get_troop_by_period(period + i);

        if i > 1 {
            let prev = get_troop_by_period(period + i - 1);
            if x == prev {
                continue
            }
        }

        let duration = Duration::hours(i as i64);

        vec.push(Event{
            started_at: dt + duration,
            troop: x
        })
    }

    vec
}


