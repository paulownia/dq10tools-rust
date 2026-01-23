use chrono::{DateTime, TimeZone, Utc, NaiveDateTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ColorStyle {
    Standard(u32),
    Extended(u32),
    Rainbow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Troop {
    name: &'static str,
    color: ColorStyle,
}

impl Troop {
    const fn standard(name: &'static str, code: u32) -> Self {
        Self { name, color: ColorStyle::Standard(code) }
    }

    const fn extended(name: &'static str, code: u32) -> Self {
        Self { name, color: ColorStyle::Extended(code) }
    }

    const fn rainbow(name: &'static str) -> Self {
        Self { name, color: ColorStyle::Rainbow }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn colorized_name(&self) -> String {
        match self.color {
            ColorStyle::Standard(code) => {
                format!("\x1b[{}m{}\x1b[0m", code, self.name)
            }
            ColorStyle::Extended(code) => {
                format!("\x1b[38;5;{}m{}\x1b[0m", code, self.name)
            }
            ColorStyle::Rainbow => {
                self.name.chars().enumerate().fold(String::new(), |res, (i, ch)| {
                    res + &format!("\x1b[{}m{}\x1b[0m", (i + 6) % 7 + 31, ch)
                })
            }
        }
    }
}

const BEAST:     Troop = Troop::standard("闇朱の獣牙兵団", 31);
const MACHINE:   Troop = Troop::standard("紫炎の鉄機兵団", 35);
const GOLEM:     Troop = Troop::standard("深碧の造魔兵団", 32);
const ZOMBIE:    Troop = Troop::standard("蒼怨の屍獄兵団", 34);
const INSECT:    Troop = Troop::standard("銀甲の凶蟲兵団", 33);
const MARINE:    Troop = Troop::standard("翠煙の海妖兵団", 36);
const DRAGON:    Troop = Troop::standard("灰塵の竜鱗兵団", 37);
const SLIME:     Troop = Troop::rainbow("彩虹の粘塊兵団");
const FLOWER:    Troop = Troop::extended("芳墨の華烈兵団", 88);
const BIRD:      Troop = Troop::standard("白雲の冥翼兵団", 0);
const WOOD:      Troop = Troop::extended("腐緑の樹葬兵団", 100);
const ALL:       Troop = Troop::standard("全兵団", 1);
const VEGETABLE: Troop = Troop::extended("青鮮の菜果兵団", 190);
const STEEL:     Troop = Troop::extended("鋼塊の重滅兵団", 103);
const GOLD:      Troop = Troop::extended("金神の遺宝兵団", 220);
const GANG:      Troop = Troop::extended("紅爆の暴賊兵団", 124);

// 2025-12-10 6:00を起点とするスケジュール
const CYCLE: [Troop; 30] = [
    GOLD,
    GANG,
    ALL,
    BEAST,
    ZOMBIE,
    DRAGON,
    BIRD,
    STEEL,
    GANG,
    ALL,
    GOLD,
    GANG,
    ALL,
    MACHINE,
    INSECT,
    SLIME,
    WOOD,
    STEEL,
    GANG,
    ALL,
    GOLD,
    GANG,
    ALL,
    GOLEM,
    MARINE,
    FLOWER,
    VEGETABLE,
    STEEL,
    GANG,
    ALL,
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

pub fn get_troop_by_period(p: usize) -> Troop {
    let index = p % CYCLE.len();
    CYCLE[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(MARINE, MARINE);
        assert_eq!(DRAGON, DRAGON);
        assert_eq!(SLIME, SLIME);
        assert_ne!(MARINE, SLIME);
        assert_ne!(SLIME, DRAGON);
        assert_ne!(DRAGON, MARINE);
    }

    #[test]
    fn test_eq_same_name_different_color() {
        let a = Troop::standard("a", 1);
        let b = Troop::standard("a", 2);

        // 色が違えば別の兵団
        assert_ne!(a, b);
    }
}
