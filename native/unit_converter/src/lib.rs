mod quantity;

use quantity::{time::{Time}, QuantityUnit};

fn secs_to_mins(x: f64) -> f64{
  let seconds = Time::from_secs(x);
  let minutes = Time::to_minutes(seconds);
  println!("minutes: {:#?}", minutes);
  let unit = minutes.to_unit();
  let v = unit.value();
  let t = unit.title();
  println!("Title: {t}");
  v
}

fn secs_to_hours(x: f64) -> f64{
  let seconds = Time::from_secs(x);
  let hours = Time::to_hours(seconds);
  println!("hours: {:#?}", hours);
  let unit = hours.to_unit();
  let v = unit.value();
  let t = unit.title();
  println!("Title: {t}");
  v
}

fn secs_to_days(x: f64) -> f64{
  let seconds = Time::from_secs(x);
  let days = Time::to_days(seconds);
  println!("days: {:#?}", days);
  let unit = days.to_unit();
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

  #[test]
  fn test_seconds_to_hours() {
    assert_eq!(4_f64, secs_to_hours(14400_f64));
  }

  #[test]
  fn test_seconds_to_days() {
    assert_eq!(7_f64, secs_to_days(604800_f64));
  }
}