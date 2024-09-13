use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("ms".to_string(), "ms".to_string()), MILLIS_TO_MILLIS),
    (("ms".to_string(), "s".to_string()), MILLIS_TO_SECS),
    (("ms".to_string(), "min".to_string()), MILLIS_TO_MINS),
    (("ms".to_string(), "h".to_string()), MILLIS_TO_HOURS),
    (("ms".to_string(), "d".to_string()), MILLIS_TO_DAYS),

    (("s".to_string(), "ms".to_string()), SECS_TO_MILLIS),
    (("s".to_string(), "s".to_string()), SECS_TO_SECS),
    (("s".to_string(), "min".to_string()), SECS_TO_MINS),
    (("s".to_string(), "h".to_string()), SECS_TO_HOURS),
    (("s".to_string(), "d".to_string()), SECS_TO_DAYS),

    (("min".to_string(), "ms".to_string()), MINS_TO_MILLIS),
    (("min".to_string(), "s".to_string()), MINS_TO_SECS),
    (("min".to_string(), "min".to_string()), MINS_TO_MINS),
    (("min".to_string(), "h".to_string()), MINS_TO_HOURS),
    (("min".to_string(), "d".to_string()), MINS_TO_DAYS),

    (("h".to_string(), "ms".to_string()), HOURS_TO_MILLIS),
    (("h".to_string(), "s".to_string()), HOURS_TO_SECS),
    (("h".to_string(), "min".to_string()), HOURS_TO_MINS),
    (("h".to_string(), "h".to_string()), HOURS_TO_HOURS),
    (("h".to_string(), "d".to_string()), HOURS_TO_DAYS),
    
    (("d".to_string(), "ms".to_string()), DAYS_TO_MILLIS),
    (("d".to_string(), "s".to_string()), DAYS_TO_SECS),
    (("d".to_string(), "min".to_string()), DAYS_TO_MINS),
    (("d".to_string(), "h".to_string()), DAYS_TO_HOURS),
    (("d".to_string(), "d".to_string()), DAYS_TO_DAYS)
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

  fn to(self, other: Self) -> Self {
    let convert = CONVMAP
      .get(&(self.symbol, other.symbol.clone()))
      .unwrap();
    let result = convert(self.value);
  
    TimeUnit::with_value(result, &other.symbol)
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

  pub fn from_hours(x: f64) -> Self {
    Time::Hours(TimeUnit::with_value(x, "h"))
  }

  pub fn from_days(x: f64) -> Self {
    Time::Days(TimeUnit::with_value(x, "d"))
  }


  pub fn to_millis(self) -> Self {
    let unit_millis: TimeUnit = TimeUnit::from_symbol("ms");
    let result: TimeUnit = Time::to_unit(self).to(unit_millis);
      
    Self::Milliseconds(result)
  }

  pub fn to_secs(self) -> Self {
    let unit_secs: TimeUnit = TimeUnit::from_symbol("s");
    let result: TimeUnit = Time::to_unit(self).to(unit_secs);
      
    Self::Seconds(result)
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
}




