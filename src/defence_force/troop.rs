use chrono::{DateTime, TimeZone, Utc, NaiveDateTime};

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

impl SingleColored {
    const fn new(name: &'static str, color_code: u32) -> Self {
        Self{ name, color_code }
    }
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
struct Single256Colored {
    name: &'static str,
    color_code: u32
}

impl Single256Colored {
    const fn new(name: &'static str, color_code: u32) -> Self {
        Self{ name, color_code }
    }
}

impl Troop for Single256Colored {
    fn colorized_name(&self) -> String {
        format!("[38;5;{}m{}[0m", self.color_code, self.name)
    }
    fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(PartialEq, Eq)]
struct RainbowColored {
    name: &'static str
}

impl RainbowColored {
    const fn new(name: &'static str) -> Self {
        Self { name }
    }
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

const BEAST:   SingleColored = SingleColored::new("闇朱の獣牙兵団", 31);
const MACHINE: SingleColored = SingleColored::new("紫炎の鉄機兵団", 35);
const GOLEM:   SingleColored = SingleColored::new("深碧の造魔兵団", 32);
const ZOMBIE:  SingleColored = SingleColored::new("蒼怨の屍獄兵団", 34);
const INSECT:  SingleColored = SingleColored::new("銀甲の凶蟲兵団", 33);
const MARINE:  SingleColored = SingleColored::new("翠煙の海妖兵団", 36);
const DRAGON:  SingleColored = SingleColored::new("灰塵の竜鱗兵団", 37);
const SLIME:   RainbowColored = RainbowColored::new("彩虹の粘塊兵団");
const FLOWER:  Single256Colored = Single256Colored::new("芳墨の華烈兵団", 88);
const BIRD:    SingleColored = SingleColored::new("白雲の冥翼兵団", 0);
const WOOD:    Single256Colored = Single256Colored::new("腐緑の樹葬兵団", 100);
const ALL:     SingleColored = SingleColored::new("全兵団", 1);
const VEGETABLE: Single256Colored = Single256Colored::new("青鮮の菜果兵団", 190);
const STEEL:     Single256Colored = Single256Colored::new("鋼塊の重滅兵団", 103);
const GOLD:      Single256Colored = Single256Colored::new("金神の遺宝兵団", 220);
const GANG:      Single256Colored = Single256Colored::new("紅爆の暴賊兵団", 124);

// 2025-12-10 6:00を起点とするスケジュール
const CYCLE: [& dyn Troop; 30] = [
    &GOLD,
    &GANG,
    &ALL,
    &BEAST,
    &ZOMBIE,
    &DRAGON,
    &BIRD,
    &STEEL,
    &GANG,
    &ALL,
    &GOLD,
    &GANG,
    &ALL,
    &MACHINE,
    &INSECT,
    &SLIME,
    &WOOD,
    &STEEL,
    &GANG,
    &ALL,
    &GOLD,
    &GANG,
    &ALL,
    &GOLEM,
    &MARINE,
    &FLOWER,
    &VEGETABLE,
    &STEEL,
    &GANG,
    &ALL,
];

pub fn get_base_point() -> NaiveDateTime {
    // JSTの2025-12-10 6時 = UTCの2025-12-09 21時
    Utc.with_ymd_and_hms(2025, 12, 9, 21, 0, 0).unwrap().naive_utc()
}

pub fn calc_period<Tz: TimeZone>(dt: &DateTime<Tz>) -> Result<usize, String> {
    let base_point = get_base_point();
    let start_time = dt.naive_utc();
    if start_time < base_point {
        return Err(format!("no data before {}", base_point.format("%F %T")));
    }
    let idx = (start_time - base_point).num_hours() as usize;
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
        let a: & dyn Troop = &SingleColored::new("a", 1);
        let b: & dyn Troop = &SingleColored::new("a", 2);
        let c: & dyn Troop = &SingleColored::new("a", 3);

        // PartialEq
        assert!(a == b);
        assert!(b == c);
        assert!(c == a);
        assert!(b == a);

        // Eq
        assert!(a == a);
    }
}
