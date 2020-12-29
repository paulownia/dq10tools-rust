use chrono::{DateTime, Local, TimeZone};

pub trait Troop {
    fn colorized_name(&self) -> String;

    fn name(&self) -> &'static str;
}

pub struct SingleColored(&'static str, u32);

pub struct RainbowColored(&'static str);

impl Troop for SingleColored {
    fn colorized_name(&self) -> String {
        format!("[{}m{}[0m", self.1, self.0)
    }
    fn name(&self) -> &'static str {
        self.0
    }
}
impl Troop for RainbowColored {
    fn colorized_name(&self) -> String {
        self.0.chars().enumerate().fold(String::new(), |res, (i, ch)| {
            res + &format!("[{}m{}[0m", (i + 6) % 7 + 31, ch)
        })
    }
    fn name(&self) -> &'static str {
        self.0
    }
}
impl PartialEq for Troop {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

const BEAST:   SingleColored = SingleColored("闇朱の獣牙兵団", 31);
const MACHINE: SingleColored = SingleColored("紫炎の鉄機兵団", 35);
const GOLEM:   SingleColored = SingleColored("深碧の造魔兵団", 32);
const ZONBIE:  SingleColored = SingleColored("蒼怨の屍獄兵団", 34);
const INSECT:  SingleColored = SingleColored("銀甲の凶蟲兵団", 33);
const MARINE:  SingleColored = SingleColored("翠煙の海妖兵団", 36);
const DRAGON:  SingleColored = SingleColored("灰塵の竜鱗兵団", 37);
const SLIME:   RainbowColored = RainbowColored("彩虹の粘塊兵団");
const ALL:     SingleColored = SingleColored("全兵団", 1);

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
