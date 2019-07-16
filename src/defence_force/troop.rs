use chrono::{TimeZone, DateTime, Local, Duration, Datelike, Timelike};

#[derive(Eq, PartialEq)]
pub struct Troop(&'static str, u32);

impl Troop {
    pub fn colorized_name(&self) -> String {
        let s = format!("[{}m{}[0m", self.0, self.1);
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

pub struct State {
    pub troop: &'static Troop,
    pub next_troop: &'static Troop,
    pub next_in: u32,
    pub changed_at: chrono::DateTime<Local>
}

pub fn get_current_state() -> State {
    get_state(Local::now())
}

fn calc_period(dt: DateTime<Local>) -> usize {
    (dt.weekday().num_days_from_sunday() * 24 + dt.hour()) as usize
}

fn get_troop_by_period(p: usize) -> &'static Troop {
    let index = p % CYCLE.len();
    CYCLE[index]
}

fn is_same_troop(p1: usize, p2: usize) -> bool {
    get_troop_by_period(p1) == get_troop_by_period(p2)
}

pub fn get_state(dt: DateTime<Local>) -> State {
    let period = calc_period(dt);

    let mut next_in = 60 - dt.minute();

    let mut next_period = period + 1;

    while is_same_troop(period, next_period) {
        next_period = (next_period + 1) % CYCLE.len();
        next_in += 60;
    }

    let changed_at = dt.clone() + Duration::minutes(next_in as i64);

    State {
        troop: get_troop_by_period(period),
        next_troop: get_troop_by_period(next_period),
        next_in: next_in,
        changed_at: changed_at
    }
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


#[test]
fn test_get_state() {
    let dt = Local.ymd(2018, 9, 20).and_hms(15, 3, 15);
    let state = get_state(dt);
    assert_eq!(state.troop.name(), "闇朱の獣牙兵団");
    assert_eq!(state.next_troop.name(), "紫炎の鉄機兵団");

}
