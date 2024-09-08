mod quantity;

use quantity::time::{Time, TimeTrait};

fn secs_to_mins(x: f64) -> f64{
  let seconds = Time::from_secs(120f64);
  let minutes = Time::to_minutes(seconds);
  println!("minutes: {:#?}", minutes);
  let unit = minutes.value();
  let v = unit.value();
  let t = unit.title();
  println!("Title: {t}");
  v
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_seconds_to_minutes() {
    assert_eq!(2_f64, secs_to_mins(120_f64));
  }
}