use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    ()
  ])
});

#[derive(Debug)]
pub struct LengthUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit for LengthUnit {
  fn from_symbol(sym: &str) -> Self {
    match sym {
      mm @ "mm" => Self {
        value: 0_f64,
        title: String::from("Millimetres"),
        symbol: String::from(mm)
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

    LengthUnit::with_value(result, &other.symbol)
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

