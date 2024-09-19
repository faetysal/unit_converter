use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("us_gal".to_string(), "us_gal".to_string()), USGAL_TO_USGAL),
    (("us_gal".to_string(), "uk_gal".to_string()), USGAL_TO_UKGAL),
    (("us_gal".to_string(), "l".to_string()), USGAL_TO_L),
    (("us_gal".to_string(), "ml".to_string()), USGAL_TO_ML),
    (("us_gal".to_string(), "cm^3".to_string()), USGAL_TO_CM3),
    (("us_gal".to_string(), "m^3".to_string()), USGAL_TO_M3),
    (("us_gal".to_string(), "in^3".to_string()), USGAL_TO_IN3),
    (("us_gal".to_string(), "ft^3".to_string()), USGAL_TO_FT3),
    
    (("uk_gal".to_string(), "us_gal".to_string()), UKGAL_TO_USGAL),
    (("uk_gal".to_string(), "uk_gal".to_string()), UKGAL_TO_UKGAL),
    (("uk_gal".to_string(), "l".to_string()), UKGAL_TO_L),
    (("uk_gal".to_string(), "ml".to_string()), UKGAL_TO_ML),
    (("uk_gal".to_string(), "cm^3".to_string()), UKGAL_TO_CM3),
    (("uk_gal".to_string(), "m^3".to_string()), UKGAL_TO_M3),
    (("uk_gal".to_string(), "in^3".to_string()), UKGAL_TO_IN3),
    (("uk_gal".to_string(), "ft^3".to_string()), UKGAL_TO_FT3),

    (("l".to_string(), "us_gal".to_string()), L_TO_USGAL),
    (("l".to_string(), "uk_gal".to_string()), L_TO_UKGAL),
    (("l".to_string(), "l".to_string()), L_TO_L),
    (("l".to_string(), "ml".to_string()), L_TO_ML),
    (("l".to_string(), "cm^3".to_string()), L_TO_CM3),
    (("l".to_string(), "m^3".to_string()), L_TO_M3),
    (("l".to_string(), "in^3".to_string()), L_TO_IN3),
    (("l".to_string(), "ft^3".to_string()), L_TO_FT3),

    (("ml".to_string(), "us_gal".to_string()), ML_TO_USGAL),
    (("ml".to_string(), "uk_gal".to_string()), ML_TO_UKGAL),
    (("ml".to_string(), "l".to_string()), ML_TO_L),
    (("ml".to_string(), "ml".to_string()), ML_TO_ML),
    (("ml".to_string(), "cm^3".to_string()), ML_TO_CM3),
    (("ml".to_string(), "m^3".to_string()), ML_TO_M3),
    (("ml".to_string(), "in^3".to_string()), ML_TO_IN3),
    (("ml".to_string(), "ft^3".to_string()), ML_TO_FT3),

    (("cm^3".to_string(), "us_gal".to_string()), CM3_TO_USGAL),
    (("cm^3".to_string(), "uk_gal".to_string()), CM3_TO_UKGAL),
    (("cm^3".to_string(), "l".to_string()), CM3_TO_L),
    (("cm^3".to_string(), "ml".to_string()), CM3_TO_ML),
    (("cm^3".to_string(), "cm^3".to_string()), CM3_TO_CM3),
    (("cm^3".to_string(), "m^3".to_string()), CM3_TO_M3),
    (("cm^3".to_string(), "in^3".to_string()), CM3_TO_IN3),
    (("cm^3".to_string(), "ft^3".to_string()), CM3_TO_FT3),

    (("m^3".to_string(), "us_gal".to_string()), M3_TO_USGAL),
    (("m^3".to_string(), "uk_gal".to_string()), M3_TO_UKGAL),
    (("m^3".to_string(), "l".to_string()), M3_TO_L),
    (("m^3".to_string(), "ml".to_string()), M3_TO_ML),
    (("m^3".to_string(), "cm^3".to_string()), M3_TO_CM3),
    (("m^3".to_string(), "m^3".to_string()), M3_TO_M3),
    (("m^3".to_string(), "in^3".to_string()), M3_TO_IN3),
    (("m^3".to_string(), "ft^3".to_string()), M3_TO_FT3),

    (("in^3".to_string(), "us_gal".to_string()), IN3_TO_USGAL),
    (("in^3".to_string(), "uk_gal".to_string()), IN3_TO_UKGAL),
    (("in^3".to_string(), "l".to_string()), IN3_TO_L),
    (("in^3".to_string(), "ml".to_string()), IN3_TO_ML),
    (("in^3".to_string(), "cm^3".to_string()), IN3_TO_CM3),
    (("in^3".to_string(), "m^3".to_string()), IN3_TO_M3),
    (("in^3".to_string(), "in^3".to_string()), IN3_TO_IN3),
    (("in^3".to_string(), "ft^3".to_string()), IN3_TO_FT3),

    (("ft^3".to_string(), "us_gal".to_string()), FT3_TO_USGAL),
    (("ft^3".to_string(), "uk_gal".to_string()), FT3_TO_UKGAL),
    (("ft^3".to_string(), "l".to_string()), FT3_TO_L),
    (("ft^3".to_string(), "ml".to_string()), FT3_TO_ML),
    (("ft^3".to_string(), "cm^3".to_string()), FT3_TO_CM3),
    (("ft^3".to_string(), "m^3".to_string()), FT3_TO_M3),
    (("ft^3".to_string(), "in^3".to_string()), FT3_TO_IN3),
    (("ft^3".to_string(), "ft^3".to_string()), FT3_TO_FT3)
  ])
});

#[derive(Debug)]
pub struct VolumeUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit for VolumeUnit {
  fn from_symbol(sym: &str) -> Self {
    match sym {
      ukgal @ "uk_gal" => Self {
        value: 0_f64,
        title: String::from("UK gallons"),
        symbol: String::from(ukgal)
      },
      usgal @ "us_gal" => Self {
        value: 0_f64,
        title: String::from("US gallons"),
        symbol: String::from(usgal)
      },
      l @ "l" => Self {
        value: 0_f64,
        title: String::from("Litres"),
        symbol: String::from(l)
      },
      ml @ "ml" => Self {
        value: 0_f64,
        title: String::from("Millilitres"),
        symbol: String::from(ml)
      },
      cm3 @ "cm^3" => Self {
        value: 0_f64,
        title: String::from("Cubic metres"),
        symbol: String::from(cm3)
      },
      m3 @ "m^3" => Self {
        value: 0_f64,
        title: String::from("Cubic metre"),
        symbol: String::from(m3)
      },
      in3 @ "in^3" => Self {
        value: 0_f64,
        title: String::from("Cubic inches"),
        symbol: String::from(in3)
      },
      ft3 @ "ft^3" => Self {
        value: 0_f64,
        title: String::from("Cubic feet"),
        symbol: String::from(ft3)
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