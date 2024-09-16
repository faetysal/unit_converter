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
      wk @ "wk" => Self {
        value: 0_f64,
        title: String::from("Weeks"),
        symbol: String::from(wk)
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

impl TimeUnit {
  pub fn from_millis(value: f64) -> Self {
    Self::with_value(value, "ms")
  }

  pub fn from_secs(value: f64) -> Self {
    Self::with_value(value, "s")
  }

  pub fn from_mins(value: f64) -> Self {
    Self::with_value(value, "min")
  }

  pub fn from_hours(value: f64) -> Self {
    Self::with_value(value, "h")
  }

  pub fn from_days(value: f64) -> Self {
    Self::with_value(value, "d")
  }

  pub fn from_weeks(value: f64) -> Self {
    Self::with_value(value, "wk")
  }

  pub fn to_millis(self) -> Self {
    self.to(TimeUnit::from_symbol("ms"))
  }

  pub fn to_secs(self) -> Self {
    self.to(TimeUnit::from_symbol("s"))
  }

  pub fn to_mins(self) -> Self {
    self.to(TimeUnit::from_symbol("min"))
  }

  pub fn to_hours(self) -> Self {
    self.to(TimeUnit::from_symbol("h"))
  }

  pub fn to_days(self) -> Self {
    self.to(TimeUnit::from_symbol("d"))
  }

  pub fn to_weeks(self) -> Self {
    self.to(TimeUnit::from_symbol("wk"))
  }
}