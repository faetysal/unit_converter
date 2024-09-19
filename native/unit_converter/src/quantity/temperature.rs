use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("°C".to_string(), "°C".to_string()), C_TO_C),
    (("°C".to_string(), "°F".to_string()), C_TO_F),
    (("°C".to_string(), "K".to_string()), C_TO_K),

    (("°F".to_string(), "°C".to_string()), F_TO_C),
    (("°F".to_string(), "°F".to_string()), F_TO_F),
    (("°F".to_string(), "K".to_string()), F_TO_K),

    (("K".to_string(), "°C".to_string()), K_TO_C),
    (("K".to_string(), "°F".to_string()), K_TO_F),
    (("K".to_string(), "K".to_string()), K_TO_K),
  ])
});

// #[derive(Hash, Eq, PartialEq, Debug)]
#[derive(Debug)]
pub struct TemperatureUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit for TemperatureUnit {
  fn from_symbol(sym: &str) -> Self {
    match sym {
      c @ "°C" => Self {
        value: 0_f64,
        title: String::from("Celcius"),
        symbol: String::from(c)
      },
      f @ "°F" => Self {
        value: 0_f64,
        title: String::from("Fahrenheit"),
        symbol: String::from(f)
      },
      k @ "K" => Self {
        value: 0_f64,
        title: String::from("Kelvin"),
        symbol: String::from(k)
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
  
    Self::with_value(result, &other.symbol)
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