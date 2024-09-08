// use std::ops::{Add, Sub, Mul, Div};
use std::marker::Copy;

pub mod time;

// trait Number: Copy + Add + Sub + Mul + Div {}

#[derive(Debug)]
pub struct QuantityUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit {
  fn from_symbol(value: f64, sym: &str) -> Self {
    match sym {
      mm @ "mm" => Self {
        value,
        title: String::from("Millimetres"),
        symbol: String::from(mm)
      },
      cm @ "cm" => Self {
        value,
        title: String::from("Centimetres"),
        symbol: String::from(cm)
      },
      // ...
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

  pub fn value(&self) -> f64 {
    self.value
  }

  pub fn title(&self) -> &str {
    &self.title
  }

  pub fn symbol(&self) -> &str {
    &self.symbol
  }
}