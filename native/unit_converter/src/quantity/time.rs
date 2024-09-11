use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("s".to_string(), "min".to_string()), SECS_TO_MINS),
    (("s".to_string(), "h".to_string()), SECS_TO_HOURS),
    (("s".to_string(), "d".to_string()), SECS_TO_DAYS),
  ])
});

// #[derive(Hash, Eq, PartialEq, Debug)]
#[derive(Debug)]
pub struct TimeUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit for TimeUnit {
  fn from_symbol(sym: &str) -> Self {
    match sym {
      ms @ "ms" => Self {
        value: 0_f64,
        title: String::from("Milliseconds"),
        symbol: String::from(ms)
      },
      s @ "s" => Self {
        value: 0_f64,
        title: String::from("Seconds"),
        symbol: String::from(s)
      },
      min @ "min" => Self {
        value: 0_f64,
        title: String::from("Minutes"),
        symbol: String::from(min)
      },
      h @ "h" => Self {
        value: 0_f64,
        title: String::from("Hours"),
        symbol: String::from(h)
      },
      d @ "d" => Self {
        value: 0_f64,
        title: String::from("Days"),
        symbol: String::from(d)
      },
      _ => panic!("Invalid symbol")
    }
  }

  fn with_value(value: f64, sym: &str) -> Self {
    let mut unit = Self::from_symbol(sym);
    unit.value = value;
    unit
  }

  fn value(&self) -> f64 {
    self.value
  }

  fn title(&self) -> &str {
    &self.title
  }

  fn symbol(&self) -> &str {
    &self.symbol
  }
}

impl TimeUnit {
  fn to(self, other: Self) -> Self {
    let convert = CONVMAP
      .get(&(self.symbol, other.symbol.clone()))
      .unwrap();
    let result = convert(self.value);

    TimeUnit::with_value(result, &other.symbol)
  }
}

#[derive(Debug)]
pub enum Time {
  Milliseconds(TimeUnit),
  Seconds(TimeUnit),
  Minutes(TimeUnit),
  Hours(TimeUnit),
  Days(TimeUnit)
}

impl Time {
  pub fn to_unit(self) -> TimeUnit {
    match self {
      Time::Milliseconds(u) |
      Time::Seconds(u) |
      Time::Minutes(u) |
      Time::Hours(u) |
      Time::Days(u) 
      => u
    }
  }

  pub fn from_millis(x: f64) -> Self {
    Time::Milliseconds(TimeUnit::with_value(x, "ms"))
  }

  pub fn from_secs(x: f64) -> Self {
    Time::Seconds(TimeUnit::with_value(x, "s"))
  }

  pub fn from_mins(x: f64) -> Self {
    Time::Minutes(TimeUnit::with_value(x, "min"))
  }


  pub fn to_minutes(self) -> Self {
    let unit_mins: TimeUnit = TimeUnit::from_symbol("min");
    let result: TimeUnit = Time::to_unit(self).to(unit_mins);
      
     Self::Minutes(result)
  }
    
  pub fn to_hours(self) -> Self {
    let unit_hours: TimeUnit = TimeUnit::from_symbol("h");
    let result: TimeUnit = Time::to_unit(self).to(unit_hours);
  
    Self::Hours(result)
  }

  pub fn to_days(self) -> Self {
    let unit_days: TimeUnit = TimeUnit::from_symbol("d");
    let result: TimeUnit = Time::to_unit(self).to(unit_days);
  
    Self::Days(result)
  }


  fn millis_to_millis(x: f64) -> f64 { 0_f64 }
  fn millis_to_secs(x: f64) -> f64 { 0_f64 }
  fn millis_to_mins(x: f64) -> f64 { 0_f64 }
  fn millis_to_hours(x: f64) -> f64 { 0_f64 }
  fn millis_to_days(x: f64) -> f64 { 0_f64 }

  fn secs_to_millis(x: f64) -> f64 { 0_f64 }
  fn secs_to_secs(x: f64) -> f64 { 0_f64 }
  fn secs_to_days(x: f64) -> f64 { 0_f64 }

  fn mins_to_millis(x: f64) -> f64 { 0_f64 }
  fn mins_to_secs(x: f64) -> f64 { 0_f64 }
  fn mins_to_mins(x: f64) -> f64 { 0_f64 }
  fn mins_to_hours(x: f64) -> f64 { 0_f64 }
  fn mins_to_days(x: f64) -> f64 { 0_f64 }

  fn hours_to_millis(x: f64) -> f64 { 0_f64 }
  fn hours_to_secs(x: f64) -> f64 { 0_f64 }
  fn hours_to_mins(x: f64) -> f64 { 0_f64 }
  fn hours_to_hours(x: f64) -> f64 { 0_f64 }
  fn hours_to_days(x: f64) -> f64 { 0_f64 }

  fn days_to_millis(x: f64) -> f64 { 0_f64 }
  fn days_to_secs(x: f64) -> f64 { 0_f64 }
  fn days_to_mins(x: f64) -> f64 { 0_f64 }
  fn days_to_hours(x: f64) -> f64 { 0_f64 }
  fn days_to_days(x: f64) -> f64 { 0_f64 }
}




