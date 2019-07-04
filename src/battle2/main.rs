use std::time::SystemTime;
use serde_json::json;

fn main() {
  const MILLIS_ONE_DAY : u64 = 86400000;
  const BASE_POINT : u64 = 1524171600000;  // 2018-04-20 06:00:00
  match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => {
        let d = ((n.as_millis() as u64) - BASE_POINT) / MILLIS_ONE_DAY;
        let d0 = d % 3 + 1;
        let d2 = (d + 2) % 3 + 1;
        let d1 = (d + 1) % 3 + 1;

        let json = json!({
            "レギルラッゾたち": d0,
            "スコルパイド": d2,
            "ジェルザーク": d1
        });
        println!("{}", json.to_string());
    },
    Err(_) => panic!("SystemTime before UNIX EPOCH"),
  }
}
