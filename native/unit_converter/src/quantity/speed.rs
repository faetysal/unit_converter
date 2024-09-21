use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("m/s".to_string(), "m/s".to_string()), MS_TO_MS),
    (("m/s".to_string(), "m/h".to_string()), MS_TO_MH),
    (("m/s".to_string(), "km/s".to_string()), MS_TO_KMS),
    (("m/s".to_string(), "km/h".to_string()), MS_TO_KMH),
    (("m/s".to_string(), "in/s".to_string()), MS_TO_INS),
    (("m/s".to_string(), "in/h".to_string()), MS_TO_INH),
    (("m/s".to_string(), "ft/s".to_string()), MS_TO_FTS),
    (("m/s".to_string(), "ft/h".to_string()), MS_TO_FTH),
    (("m/s".to_string(), "mi/s".to_string()), MS_TO_MIS),
    (("m/s".to_string(), "mi/h".to_string()), MS_TO_MIH),
    (("m/s".to_string(), "kn".to_string()), MS_TO_KN),
    
    (("m/h".to_string(), "m/s".to_string()), MH_TO_MS),
    (("m/h".to_string(), "m/h".to_string()), MH_TO_MH),
    (("m/h".to_string(), "km/s".to_string()), MH_TO_KMS),
    (("m/h".to_string(), "km/h".to_string()), MH_TO_KMH),
    (("m/h".to_string(), "in/s".to_string()), MH_TO_INS),
    (("m/h".to_string(), "in/h".to_string()), MH_TO_INH),
    (("m/h".to_string(), "ft/s".to_string()), MH_TO_FTS),
    (("m/h".to_string(), "ft/h".to_string()), MH_TO_FTH),
    (("m/h".to_string(), "mi/s".to_string()), MH_TO_MIS),
    (("m/h".to_string(), "mi/h".to_string()), MH_TO_MIH),
    (("m/h".to_string(), "kn".to_string()), MH_TO_KN),
    
    (("km/s".to_string(), "m/s".to_string()), KMS_TO_MS),
    (("km/s".to_string(), "m/h".to_string()), KMS_TO_MH),
    (("km/s".to_string(), "km/s".to_string()), KMS_TO_KMS),
    (("km/s".to_string(), "km/h".to_string()), KMS_TO_KMH),
    (("km/s".to_string(), "in/s".to_string()), KMS_TO_INS),
    (("km/s".to_string(), "in/h".to_string()), KMS_TO_INH),
    (("km/s".to_string(), "ft/s".to_string()), KMS_TO_FTS),
    (("km/s".to_string(), "ft/h".to_string()), KMS_TO_FTH),
    (("km/s".to_string(), "mi/s".to_string()), KMS_TO_MIS),
    (("km/s".to_string(), "mi/h".to_string()), KMS_TO_MIH),
    (("km/s".to_string(), "kn".to_string()), KMS_TO_KN),
    
    (("km/h".to_string(), "m/s".to_string()), KMH_TO_MS),
    (("km/h".to_string(), "m/h".to_string()), KMH_TO_MH),
    (("km/h".to_string(), "km/s".to_string()), KMH_TO_KMS),
    (("km/h".to_string(), "km/h".to_string()), KMH_TO_KMH),
    (("km/h".to_string(), "in/s".to_string()), KMH_TO_INS),
    (("km/h".to_string(), "in/h".to_string()), KMH_TO_INH),
    (("km/h".to_string(), "ft/s".to_string()), KMH_TO_FTS),
    (("km/h".to_string(), "ft/h".to_string()), KMH_TO_FTH),
    (("km/h".to_string(), "mi/s".to_string()), KMH_TO_MIS),
    (("km/h".to_string(), "mi/h".to_string()), KMH_TO_MIH),
    (("km/h".to_string(), "kn".to_string()), KMH_TO_KN),
    
    (("in/s".to_string(), "m/s".to_string()), INS_TO_MS),
    (("in/s".to_string(), "m/h".to_string()), INS_TO_MH),
    (("in/s".to_string(), "km/s".to_string()), INS_TO_KMS),
    (("in/s".to_string(), "km/h".to_string()), INS_TO_KMH),
    (("in/s".to_string(), "in/s".to_string()), INS_TO_INS),
    (("in/s".to_string(), "in/h".to_string()), INS_TO_INH),
    (("in/s".to_string(), "ft/s".to_string()), INS_TO_FTS),
    (("in/s".to_string(), "ft/h".to_string()), INS_TO_FTH),
    (("in/s".to_string(), "mi/s".to_string()), INS_TO_MIS),
    (("in/s".to_string(), "mi/h".to_string()), INS_TO_MIH),
    (("in/s".to_string(), "kn".to_string()), INS_TO_KN),
    
    (("in/h".to_string(), "m/s".to_string()), INH_TO_MS),
    (("in/h".to_string(), "m/h".to_string()), INH_TO_MH),
    (("in/h".to_string(), "km/s".to_string()), INH_TO_KMS),
    (("in/h".to_string(), "km/h".to_string()), INH_TO_KMH),
    (("in/h".to_string(), "in/s".to_string()), INH_TO_INS),
    (("in/h".to_string(), "in/h".to_string()), INH_TO_INH),
    (("in/h".to_string(), "ft/s".to_string()), INH_TO_FTS),
    (("in/h".to_string(), "ft/h".to_string()), INH_TO_FTH),
    (("in/h".to_string(), "mi/s".to_string()), INH_TO_MIS),
    (("in/h".to_string(), "mi/h".to_string()), INH_TO_MIH),
    (("in/h".to_string(), "kn".to_string()), INH_TO_KN),
    
    (("ft/s".to_string(), "m/s".to_string()), FTS_TO_MS),
    (("ft/s".to_string(), "m/h".to_string()), FTS_TO_MH),
    (("ft/s".to_string(), "km/s".to_string()), FTS_TO_KMS),
    (("ft/s".to_string(), "km/h".to_string()), FTS_TO_KMH),
    (("ft/s".to_string(), "in/s".to_string()), FTS_TO_INS),
    (("ft/s".to_string(), "in/h".to_string()), FTS_TO_INH),
    (("ft/s".to_string(), "ft/s".to_string()), FTS_TO_FTS),
    (("ft/s".to_string(), "ft/h".to_string()), FTS_TO_FTH),
    (("ft/s".to_string(), "mi/s".to_string()), FTS_TO_MIS),
    (("ft/s".to_string(), "mi/h".to_string()), FTS_TO_MIH),
    (("ft/s".to_string(), "kn".to_string()), FTS_TO_KN),
    
    (("ft/h".to_string(), "m/s".to_string()), FTH_TO_MS),
    (("ft/h".to_string(), "m/h".to_string()), FTH_TO_MH),
    (("ft/h".to_string(), "km/s".to_string()), FTH_TO_KMS),
    (("ft/h".to_string(), "km/h".to_string()), FTH_TO_KMH),
    (("ft/h".to_string(), "in/s".to_string()), FTH_TO_INS),
    (("ft/h".to_string(), "in/h".to_string()), FTH_TO_INH),
    (("ft/h".to_string(), "ft/s".to_string()), FTH_TO_FTS),
    (("ft/h".to_string(), "ft/h".to_string()), FTH_TO_FTH),
    (("ft/h".to_string(), "mi/s".to_string()), FTH_TO_MIS),
    (("ft/h".to_string(), "mi/h".to_string()), FTH_TO_MIH),
    (("ft/h".to_string(), "kn".to_string()), FTH_TO_KN),
    
    (("mi/s".to_string(), "m/s".to_string()), MIS_TO_MS),
    (("mi/s".to_string(), "m/h".to_string()), MIS_TO_MH),
    (("mi/s".to_string(), "km/s".to_string()), MIS_TO_KMS),
    (("mi/s".to_string(), "km/h".to_string()), MIS_TO_KMH),
    (("mi/s".to_string(), "in/s".to_string()), MIS_TO_INS),
    (("mi/s".to_string(), "in/h".to_string()), MIS_TO_INH),
    (("mi/s".to_string(), "ft/s".to_string()), MIS_TO_FTS),
    (("mi/s".to_string(), "ft/h".to_string()), MIS_TO_FTH),
    (("mi/s".to_string(), "mi/s".to_string()), MIS_TO_MIS),
    (("mi/s".to_string(), "mi/h".to_string()), MIS_TO_MIH),
    (("mi/s".to_string(), "kn".to_string()), MIS_TO_KN),
    
    (("mi/h".to_string(), "m/s".to_string()), MIH_TO_MS),
    (("mi/h".to_string(), "m/h".to_string()), MIH_TO_MH),
    (("mi/h".to_string(), "km/s".to_string()), MIH_TO_KMS),
    (("mi/h".to_string(), "km/h".to_string()), MIH_TO_KMH),
    (("mi/h".to_string(), "in/s".to_string()), MIH_TO_INS),
    (("mi/h".to_string(), "in/h".to_string()), MIH_TO_INH),
    (("mi/h".to_string(), "ft/s".to_string()), MIH_TO_FTS),
    (("mi/h".to_string(), "ft/h".to_string()), MIH_TO_FTH),
    (("mi/h".to_string(), "mi/s".to_string()), MIH_TO_MIS),
    (("mi/h".to_string(), "mi/h".to_string()), MIH_TO_MIH),
    (("mi/h".to_string(), "kn".to_string()), MIH_TO_KN),
    
    (("kn".to_string(), "m/s".to_string()), KN_TO_MS),
    (("kn".to_string(), "m/h".to_string()), KN_TO_MH),
    (("kn".to_string(), "km/s".to_string()), KN_TO_KMS),
    (("kn".to_string(), "km/h".to_string()), KN_TO_KMH),
    (("kn".to_string(), "in/s".to_string()), KN_TO_INS),
    (("kn".to_string(), "in/h".to_string()), KN_TO_INH),
    (("kn".to_string(), "ft/s".to_string()), KN_TO_FTS),
    (("kn".to_string(), "ft/h".to_string()), KN_TO_FTH),
    (("kn".to_string(), "mi/s".to_string()), KN_TO_MIS),
    (("kn".to_string(), "mi/h".to_string()), KN_TO_MIH),
    (("kn".to_string(), "kn".to_string()), KN_TO_KN)
  ])
});

#[derive(Debug)]
pub struct SpeedUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit for SpeedUnit {
  fn from_symbol(sym: &str) -> Self {
    match sym {
      mps @ "m/s" => Self {
        value: 0_f64,
        title: String::from("Metres per second"),
        symbol: String::from(mps)
      },
      mph @ "m/h" => Self {
        value: 0_f64,
        title: String::from("Metres per hour"),
        symbol: String::from(mph)
      },
      kms @ "km/s" => Self {
        value: 0_f64,
        title: String::from("Kilometres per second"),
        symbol: String::from(kms)
      },
      kmh @ "km/h" => Self {
        value: 0_f64,
        title: String::from("Kilometres per hour"),
        symbol: String::from(kmh)
      },
      ins @ "in/s" => Self {
        value: 0_f64,
        title: String::from("Inches per second"),
        symbol: String::from(ins)
      },
      inh @ "in/h" => Self {
        value: 0_f64,
        title: String::from("Inches per hour"),
        symbol: String::from(inh)
      },
      fts @ "ft/s" => Self {
        value: 0_f64,
        title: String::from("Feet per second"),
        symbol: String::from(fts)
      },
      fth @ "ft/h" => Self {
        value: 0_f64,
        title: String::from("Feet per hour"),
        symbol: String::from(fth)
      },
      mis @ "mi/s" => Self {
        value: 0_f64,
        title: String::from("Miles per second"),
        symbol: String::from(mis)
      },
      mih @ "mi/h" => Self {
        value: 0_f64,
        title: String::from("Miles per hour"),
        symbol: String::from(mih)
      },
      kn @ "kn" => Self {
        value: 0_f64,
        title: String::from("Knots"),
        symbol: String::from(kn)
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