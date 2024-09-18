use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("ac".to_string(), "ac".to_string()), AC_TO_AC),
    (("ac".to_string(), "a".to_string()), AC_TO_A),
    (("ac".to_string(), "ha".to_string()), AC_TO_HA),
    (("ac".to_string(), "cm^2".to_string()), AC_TO_CM2),
    (("ac".to_string(), "ft^2".to_string()), AC_TO_FT2),
    (("ac".to_string(), "in^2".to_string()), AC_TO_IN2),
    (("ac".to_string(), "m^2".to_string()), AC_TO_M2),

    (("a".to_string(), "ac".to_string()), A_TO_AC),
    (("a".to_string(), "a".to_string()), A_TO_A),
    (("a".to_string(), "ha".to_string()), A_TO_HA),
    (("a".to_string(), "cm^2".to_string()), A_TO_CM2),
    (("a".to_string(), "ft^2".to_string()), A_TO_FT2),
    (("a".to_string(), "in^2".to_string()), A_TO_IN2),
    (("a".to_string(), "m^2".to_string()), A_TO_M2),

    (("ha".to_string(), "ac".to_string()), HA_TO_AC),
    (("ha".to_string(), "a".to_string()), HA_TO_A),
    (("ha".to_string(), "ha".to_string()), HA_TO_HA),
    (("ha".to_string(), "cm^2".to_string()), HA_TO_CM2),
    (("ha".to_string(), "ft^2".to_string()), HA_TO_FT2),
    (("ha".to_string(), "in^2".to_string()), HA_TO_IN2),
    (("ha".to_string(), "m^2".to_string()), HA_TO_M2),

    (("cm^2".to_string(), "ac".to_string()), CM2_TO_AC),
    (("cm^2".to_string(), "a".to_string()), CM2_TO_A),
    (("cm^2".to_string(), "ha".to_string()), CM2_TO_HA),
    (("cm^2".to_string(), "cm^2".to_string()), CM2_TO_CM2),
    (("cm^2".to_string(), "ft^2".to_string()), CM2_TO_FT2),
    (("cm^2".to_string(), "in^2".to_string()), CM2_TO_IN2),
    (("cm^2".to_string(), "m^2".to_string()), CM2_TO_M2),

    (("ft^2".to_string(), "ac".to_string()), FT2_TO_AC),
    (("ft^2".to_string(), "a".to_string()), FT2_TO_A),
    (("ft^2".to_string(), "ha".to_string()), FT2_TO_HA),
    (("ft^2".to_string(), "cm^2".to_string()), FT2_TO_CM2),
    (("ft^2".to_string(), "ft^2".to_string()), FT2_TO_FT2),
    (("ft^2".to_string(), "in^2".to_string()), FT2_TO_IN2),
    (("ft^2".to_string(), "m^2".to_string()), FT2_TO_M2),

    (("in^2".to_string(), "ac".to_string()), IN2_TO_AC),
    (("in^2".to_string(), "a".to_string()), IN2_TO_A),
    (("in^2".to_string(), "ha".to_string()), IN2_TO_HA),
    (("in^2".to_string(), "cm^2".to_string()), IN2_TO_CM2),
    (("in^2".to_string(), "ft^2".to_string()), IN2_TO_FT2),
    (("in^2".to_string(), "in^2".to_string()), IN2_TO_IN2),
    (("in^2".to_string(), "m^2".to_string()), IN2_TO_M2),

    (("m^2".to_string(), "ac".to_string()), M2_TO_AC),
    (("m^2".to_string(), "a".to_string()), M2_TO_A),
    (("m^2".to_string(), "ha".to_string()), M2_TO_HA),
    (("m^2".to_string(), "cm^2".to_string()), M2_TO_CM2),
    (("m^2".to_string(), "ft^2".to_string()), M2_TO_FT2),
    (("m^2".to_string(), "in^2".to_string()), M2_TO_IN2),
    (("m^2".to_string(), "m^2".to_string()), M2_TO_M2)
  ])
});

// #[derive(Hash, Eq, PartialEq, Debug)]
#[derive(Debug)]
pub struct AreaUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit for AreaUnit {
  fn from_symbol(sym: &str) -> Self {
    match sym {
      ac @ "ac" => Self {
        value: 0_f64,
        title: String::from("Acres"),
        symbol: String::from(ac)
      },
      a @ "a" => Self {
        value: 0_f64,
        title: String::from("Ares"),
        symbol: String::from(a)
      },
      ha @ "ha" => Self {
        value: 0_f64,
        title: String::from("Hectares"),
        symbol: String::from(ha)
      },
      cm2 @ "cm^2" => Self {
        value: 0_f64,
        title: String::from("Square centimetres"),
        symbol: String::from(cm2)
      },
      ft2 @ "ft^2" => Self {
        value: 0_f64,
        title: String::from("Square feet"),
        symbol: String::from(ft2)
      },
      in2 @ "in^2" => Self {
        value: 0_f64,
        title: String::from("Square inches"),
        symbol: String::from(in2)
      },
      m2 @ "m^2" => Self {
        value: 0_f64,
        title: String::from("Square metres"),
        symbol: String::from(m2)
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