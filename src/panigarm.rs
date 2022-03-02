use chrono::{DateTime, Local, TimeZone};

#[derive(PartialEq, Eq)]
pub struct Boss {
   pub name: &'static str
}

const ALMANA: Boss = Boss { name: "源世鳥アルマナ" };
const DRAGON: Boss = Boss { name: "じげんりゅう" };
const CORVUS: Boss = Boss { name: "堕天使エルギオス" };

const SEQUENCE: [& Boss; 3] = [
    &ALMANA,
    &DRAGON,
    &CORVUS,
];

pub fn get_current_boss() -> &'static Boss {
    get_boss(Local::now())
}

pub fn get_boss(dt: DateTime<Local>) -> &'static Boss {
    let base_point = get_base_point();
    let duration = dt - base_point;
    let pass = duration.num_weeks() as usize;
    let index = pass % SEQUENCE.len();
    SEQUENCE[index]
}

pub fn get_base_point() -> DateTime<Local> {
    Local.ymd(2021, 12, 12).and_hms(6, 0, 0)
}



#[cfg(test)]
mod tests {
}
