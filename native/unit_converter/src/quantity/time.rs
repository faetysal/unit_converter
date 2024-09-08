use super::QuantityUnit;

pub trait TimeTrait {
  // fn to_milliseconds(&self) -> Time;
  // fn to_secs(&self) -> Time;
  fn to_minutes(self) -> Time;
  /*fn to_hours(&self) -> u32;
  fn to_days(&self) -> u32;
  fn to_weeks(&self) -> u32;*/
}

#[derive(Debug)]
pub struct TimeUnit {
  value: f64,
  title: String,
  symbol: String
}
impl QuantityUnit for TimeUnit {
  fn from_symbol(value: f64, sym: &str) -> Self {
    match sym {
      ms @ "ms" => Self {
        value,
        title: String::from("Milliseconds"),
        symbol: String::from(ms)
      },
      s @ "s" => Self {
        value,
        title: String::from("Seconds"),
        symbol: String::from(s)
      },
      min @ "min" => Self {
        value,
        title: String::from("Minutes"),
        symbol: String::from(min)
      },
      _ => panic!("Invalid symbol")
    }
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
  Minutes(TimeUnit)
}

impl Time {
  pub fn from_millis(x: f64) -> Self {
    Time::Milliseconds(TimeUnit::from_symbol(x, "ms"))
  }

  pub fn from_secs(x: f64) -> Self {
    Time::Seconds(QuantityUnit::from_symbol(x, "s"))
  }

  pub fn from_mins(x: f64) -> Self {
    Time::Minutes(QuantityUnit::from_symbol(x, "min"))
  }

  pub fn value(self) -> TimeUnit {
    match self {
      Time::Milliseconds(u) |
      Time::Seconds(u) |
      Time::Minutes(u) => u
    }
  }
}

impl TimeTrait for Time {
  fn to_minutes(self) -> Self {
    let result = match self {
      Time::Milliseconds(q) => q.value,
      Time::Seconds(q) => {
        let result = q.value / 60.0;
        result
      }
      Time::Minutes(q) => q.value,
    };

    Self::Minutes(QuantityUnit::from_symbol(result, "min"))
  }
}




