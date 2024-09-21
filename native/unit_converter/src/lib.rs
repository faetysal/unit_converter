mod quantity;

pub use quantity::{
  QuantityUnit,
  time::TimeUnit,
  length::LengthUnit,
  area::AreaUnit,
  temperature::TemperatureUnit,
  volume::VolumeUnit,
  mass::MassUnit,
  data::DataUnit,
  speed::SpeedUnit
};

fn secs_to_mins(x: f64) -> f64{
  let seconds = TimeUnit::from_secs(x);
  let minutes = seconds.to_mins();
  println!("minutes: {:#?}", minutes);
  let v = minutes.value();
  let t = minutes.title();
  println!("Title: {t}");
  v
}

/*fn secs_to_hours(x: f64) -> f64{
  let seconds = Time::from_secs(x);
  let hours = Time::to_hours(seconds);
  println!("hours: {:#?}", hours);
  let unit = hours.to_unit();
  let v = unit.value();
  let t = unit.title();
  println!("Title: {t}");
  v
}

fn secs_to_days(x: f64) -> f64 {
  let seconds = Time::from_secs(x);
  let days = Time::to_days(seconds);
  println!("days: {:#?}", days);
  let unit = days.to_unit();
  let v = unit.value();
  let t = unit.title();
  println!("Title: {t}");
  v
}

fn mm_to_m(x: f64) -> f64 {
  let mm = Length::from_mm(x);
  let m = Length::to_m(mm);
  let unit = m.to_unit();
  println!("M: {:#?}", unit);

  unit.value()
}*/

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_seconds_to_minutes() {
    assert_eq!(2_f64, secs_to_mins(120_f64));
  }

  /*#[test]
  fn test_seconds_to_hours() {
    assert_eq!(4_f64, secs_to_hours(14400_f64));
  }

  #[test]
  fn test_seconds_to_days() {
    assert_eq!(7_f64, secs_to_days(604800_f64));
  }

  #[test]
  fn test_mm_to_m() {
    assert_eq!(1.5, mm_to_m(1500_f64));
  }*/
}