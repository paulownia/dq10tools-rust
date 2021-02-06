use chrono::{DateTime, Local, TimeZone};

pub trait Troop {
    fn colorized_name(&self) -> String;

    fn name(&self) -> &'static str;
}

impl PartialEq for dyn Troop {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

#[derive(PartialEq, Eq)]
struct SingleColored {
    name: &'static str,
    color_code: u32
}

impl Troop for SingleColored {
    fn colorized_name(&self) -> String {
        format!("[{}m{}[0m", self.color_code, self.name)
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(PartialEq, Eq)]
struct RainbowColored{
    name: &'static str
}

impl Troop for RainbowColored {
    fn colorized_name(&self) -> String {
        self.name.chars().enumerate().fold(String::new(), |res, (i, ch)| {
            res + &format!("[{}m{}[0m", (i + 6) % 7 + 31, ch)
        })
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

const BEAST:   SingleColored = SingleColored{ name: "闇朱の獣牙兵団", color_code: 31 };
const MACHINE: SingleColored = SingleColored{ name: "紫炎の鉄機兵団", color_code: 35 };
const GOLEM:   SingleColored = SingleColored{ name: "深碧の造魔兵団", color_code: 32 };
const ZOMBIE:  SingleColored = SingleColored{ name: "蒼怨の屍獄兵団", color_code: 34 };
const INSECT:  SingleColored = SingleColored{ name: "銀甲の凶蟲兵団", color_code: 33 };
const MARINE:  SingleColored = SingleColored{ name: "翠煙の海妖兵団", color_code: 36 };
const DRAGON:  SingleColored = SingleColored{ name: "灰塵の竜鱗兵団", color_code: 37 };
const SLIME:   RainbowColored = RainbowColored{ name:"彩虹の粘塊兵団" };
const ALL:     SingleColored = SingleColored{ name: "全兵団", color_code: 1 };

// 2020/12/23 6時からの周期
const CYCLE: [& dyn Troop; 13] = [
    &BEAST,
    &MACHINE,
    &SLIME,
    &ALL,
    &GOLEM,
    &ZOMBIE,
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

pub fn get_troop_by_period(p: usize) -> &'static dyn Troop {
    let index = p % CYCLE.len();
    CYCLE[index]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eq() {
        let marine: & dyn Troop = &MARINE;
        let slime: & dyn Troop = &SLIME;
        let dragon: & dyn Troop = &DRAGON;

        assert!(marine == marine);
        assert!(dragon == dragon);
        assert!(slime == slime);
        assert!(marine != slime);
        assert!(slime != dragon);
        assert!(dragon != marine);
    }

    #[test]
    fn test_eq2() {
        let a: & dyn Troop = &SingleColored{ name: "a", color_code: 1 };
        let b: & dyn Troop = &SingleColored{ name: "a", color_code: 2 };
        let c: & dyn Troop = &SingleColored{ name: "a", color_code: 3 };

        // PartialEq
        assert!(a == b);
        assert!(b == c);
        assert!(c == a);
        assert!(b == a);

        // Eq
        assert!(a == a);
    }
}
