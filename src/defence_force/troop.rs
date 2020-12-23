use chrono::{DateTime, Local, TimeZone};

#[derive(Eq, PartialEq)]
pub struct Troop(&'static str, u32);

impl Troop {
    pub fn colorized_name(&self) -> String {
        format!("[{}m{}[0m", self.color(), self.name())
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
const MARINE:  Troop = Troop("翠煙の海妖兵団", 36);
const DRAGON:  Troop = Troop("灰塵の竜鱗兵団", 37);
const SLIME:   Troop = Troop("彩虹の粘塊兵団", 0);
const ALL:     Troop = Troop("全兵団", 1);

// 2020/12/23 6時からの周期
const CYCLE: [&Troop; 13] = [
    &BEAST,
    &MACHINE,
    &SLIME,
    &ALL,
    &GOLEM,
    &ZONBIE,
    &INSECT,
    &SLIME,
    &ALL,
    &MARINE,
    &DRAGON,
    &SLIME,
    &ALL
];

pub fn get_base_point() -> DateTime<Local> {
    Local.ymd(2020, 12, 23).and_hms(6, 0, 0)
}

pub fn calc_period(dt: DateTime<Local>) -> Result<usize, String> {
    let base_point = get_base_point();
    if dt < base_point {
        return Err(format!("no data before {}", base_point.format("%F %T")));
    }
    let idx = (dt - base_point).num_hours() as usize;
    Ok(idx)
}

pub fn get_troop_by_period(p: usize) -> &'static Troop {
    let index = p % CYCLE.len();
    CYCLE[index]
}
