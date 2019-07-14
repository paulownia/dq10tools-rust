use chrono::{TimeZone, DateTime, Local, Duration, Datelike, Timelike};

#[derive(PartialEq, Eq)]
pub struct Army {
    pub name: &'static str,
    pub color: u32,
}

impl Army {
    fn new(data: &X) -> Army {
        Army {
            name: data.1,
            color: data.0
        }
    }
    pub fn colorized_name(&self) -> String {
        let s = format!("[{}m{}[0m", self.color, self.name);
        return s;
    }
}

#[derive(PartialEq, Eq)]
pub struct X(u32, &'static str);

const BEAST:   X = X(31, "闇朱の獣牙兵団");
const MACHINE: X = X(35, "紫炎の鉄機兵団");
const GOLEM:   X = X(32, "深碧の造魔兵団");
const ZONBIE:  X = X(34, "蒼怨の屍獄兵団");
const INSECT:  X = X(33, "銀甲の凶蟲兵団");
const RANDOM:  X = X(1, "ランダム");

// サイクル、日曜日の0時スタートでサイクルを定義、1時間毎の敵を記述
const CYCLE: [&X; 7] = [
    &MACHINE,
    &INSECT,
    &GOLEM,
    &ZONBIE,
    &INSECT,
    &RANDOM,
    &BEAST
];

pub struct State {
    pub army: Army,
    pub next_army: Army,
    pub next_in: u32,
    pub changed_at: chrono::DateTime<Local>
}

pub fn get_current_state() -> State {
    get_state(&Local::now())
}

pub fn get_state(dt: &DateTime<Local>) -> State {
    let index = index(&dt);

    let mut next_in = 60 - dt.minute();

    let mut next_index = (index + 1) % CYCLE.len();

    while CYCLE[index] == CYCLE[next_index] {
        next_index = (next_index + 1) % CYCLE.len();
        next_in += 60;
    }

    let changed_at = dt.clone() + Duration::minutes(next_in as i64);

    return State {
        army: Army::new(&CYCLE[index]),
        next_army: Army::new(&CYCLE[next_index]),
        next_in: next_in,
        changed_at: changed_at
    }
}

fn index(dt: &DateTime<Local>) -> usize {
    ((dt.weekday().num_days_from_sunday() * 24 + dt.hour()) % CYCLE.len() as u32) as usize
}

pub struct Event {
    pub started_at: DateTime<Local>,
    pub army: Army
}

pub fn get_current_schedule() -> Vec<Event> {
    get_schedule(&Local::now())
}

pub fn get_schedule(dt: &DateTime<Local>) -> Vec<Event> {
    let mut vec: Vec<Event> = Vec::with_capacity(24);
    let index = index(&dt);

    let dt = Local.ymd(dt.year(), dt.month(), dt.day()).and_hms(dt.hour(), 0, 0);

    for i in 0..24 {
        let x = &CYCLE[(index + i) % CYCLE.len()];

        if i > 1 {
            let prev = &CYCLE[(index + i - 1) % CYCLE.len()];
            if x == prev {
                continue
            }
        }

        let duration = Duration::hours(i as i64);

        vec.push(Event{
            started_at: dt + duration,
            army: Army::new(x)
        })
    }

    vec
}
