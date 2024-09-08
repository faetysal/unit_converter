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
pub enum Time {
  Milliseconds(QuantityUnit),
  Seconds(QuantityUnit),
  Minutes(QuantityUnit)
}

impl Time {
  pub fn from_millis(x: f64) -> Self {
    Time::Milliseconds(QuantityUnit::from_symbol(x, "ms"))
  }

  pub fn from_secs(x: f64) -> Self {
    Time::Seconds(QuantityUnit::from_symbol(x, "s"))
  }

  pub fn from_mins(x: f64) -> Self {
    Time::Minutes(QuantityUnit::from_symbol(x, "min"))
  }

  pub fn value(self) -> QuantityUnit {
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




