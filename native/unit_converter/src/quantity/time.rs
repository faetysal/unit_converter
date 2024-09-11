use std::collections::HashMap;
use super::QuantityUnit;

const secs_to_mins: fn(f64) -> f64 = |x| {
  x / 60_f64
};

// #[derive(Hash, Eq, PartialEq, Debug)]
struct Conversion {
  /*from: TimeUnit,
  to: TimeUnit,*/
  fmap: HashMap<(String, String), fn(f64) -> f64>
}

impl Conversion {
  fn init() -> Self {
    let map = HashMap::from([
      (("s".to_string(), "min".to_string()), secs_to_mins)
    ]);

    Self { fmap: map }
    
  }
}

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
    let conversion = Conversion::init();
    let convert = conversion
      .fmap
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
  Hours(TimeUnit)
}

impl Time {
  pub fn to_unit(self) -> TimeUnit {
    match self {
      Time::Milliseconds(u) |
      Time::Seconds(u) |
      Time::Minutes(u) |
      Time::Hours(u)
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
    let unit_min: TimeUnit = TimeUnit::from_symbol("min");
    let result: TimeUnit = Time::to_unit(self).to(unit_min);
    
    /*let result = match self {
      Time::Milliseconds(q) => Time::millis_to_mins(q.value),
      Time::Seconds(q) => Time::secs_to_mins(q.value),
      Time::Minutes(q) => Time::mins_to_mins(q.value)
      };*/
      
      Self::Minutes(result)
    }
    
  pub fn to_hours(self) -> Self {
    let unit_hour: TimeUnit = TimeUnit::from_symbol("h");
    let result: TimeUnit = Time::to_unit(self).to(unit_hour);
  
    Self::Hours(result)
  }


  fn millis_to_millis(x: f64) -> f64 { 0_f64 }
  fn millis_to_secs(x: f64) -> f64 { 0_f64 }
  fn millis_to_mins(x: f64) -> f64 { 0_f64 }
  fn millis_to_hours(x: f64) -> f64 { 0_f64 }
  fn millis_to_days(x: f64) -> f64 { 0_f64 }

  fn secs_to_millis(x: f64) -> f64 { 0_f64 }
  fn secs_to_secs(x: f64) -> f64 { 0_f64 }
  fn secs_to_mins(x: f64) -> f64 {
    x / 60_f64
  }
  fn secs_to_hours(x: f64) -> f64 { 0_f64 }
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




