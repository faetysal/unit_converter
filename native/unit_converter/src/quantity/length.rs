use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("mm".to_string(), "mm".to_string()), MM_TO_MM),
    (("mm".to_string(), "cm".to_string()), MM_TO_CM),
    (("mm".to_string(), "m".to_string()), MM_TO_M),
    (("mm".to_string(), "km".to_string()), MM_TO_KM),
    (("mm".to_string(), "in".to_string()), MM_TO_IN),
    (("mm".to_string(), "ft".to_string()), MM_TO_FT),
    (("mm".to_string(), "yd".to_string()), MM_TO_YD),
    (("mm".to_string(), "mi".to_string()), MM_TO_MI),
    (("mm".to_string(), "nm".to_string()), MM_TO_NM),
    (("mm".to_string(), "mil".to_string()), MM_TO_MIL),
    (("cm".to_string(), "mm".to_string()), CM_TO_MM),
    (("cm".to_string(), "cm".to_string()), CM_TO_CM),
    (("cm".to_string(), "m".to_string()), CM_TO_M),
    (("cm".to_string(), "km".to_string()), CM_TO_KM),
    (("cm".to_string(), "in".to_string()), CM_TO_IN),
    (("cm".to_string(), "ft".to_string()), CM_TO_FT),
    (("cm".to_string(), "yd".to_string()), CM_TO_YD),
    (("cm".to_string(), "mi".to_string()), CM_TO_MI),
    (("cm".to_string(), "nm".to_string()), CM_TO_NM),
    (("cm".to_string(), "mil".to_string()), CM_TO_MIL),
    (("m".to_string(), "mm".to_string()), M_TO_M),
    (("m".to_string(), "cm".to_string()), M_TO_CM),
    (("m".to_string(), "m".to_string()), M_TO_M),
    (("m".to_string(), "km".to_string()), M_TO_KM),
    (("m".to_string(), "in".to_string()), M_TO_IN),
    (("m".to_string(), "ft".to_string()), M_TO_FT),
    (("m".to_string(), "yd".to_string()), M_TO_YD),
    (("m".to_string(), "mi".to_string()), M_TO_MI),
    (("m".to_string(), "nm".to_string()), M_TO_NM),
    (("m".to_string(), "mil".to_string()), M_TO_MIL),
    (("km".to_string(), "mm".to_string()), KM_TO_MM),
    (("km".to_string(), "cm".to_string()), KM_TO_CM),
    (("km".to_string(), "m".to_string()), KM_TO_M),
    (("km".to_string(), "km".to_string()), KM_TO_KM),
    (("km".to_string(), "in".to_string()), KM_TO_IN),
    (("km".to_string(), "ft".to_string()), KM_TO_FT),
    (("km".to_string(), "yd".to_string()), KM_TO_YD),
    (("km".to_string(), "mi".to_string()), KM_TO_MI),
    (("km".to_string(), "nm".to_string()), KM_TO_NM),
    (("km".to_string(), "mil".to_string()), KM_TO_MIL),
    (("in".to_string(), "mm".to_string()), IN_TO_MM),
    (("in".to_string(), "cm".to_string()), IN_TO_CM),
    (("in".to_string(), "m".to_string()), IN_TO_M),
    (("in".to_string(), "km".to_string()), IN_TO_KM),
    (("in".to_string(), "in".to_string()), IN_TO_IN),
    (("in".to_string(), "ft".to_string()), IN_TO_FT),
    (("in".to_string(), "yd".to_string()), IN_TO_YD),
    (("in".to_string(), "mi".to_string()), IN_TO_MI),
    (("in".to_string(), "nm".to_string()), IN_TO_NM),
    (("in".to_string(), "mil".to_string()), IN_TO_MIL),
    (("ft".to_string(), "mm".to_string()), FT_TO_MM),
    (("ft".to_string(), "cm".to_string()), FT_TO_CM),
    (("ft".to_string(), "m".to_string()), FT_TO_M),
    (("ft".to_string(), "km".to_string()), FT_TO_KM),
    (("ft".to_string(), "in".to_string()), FT_TO_IN),
    (("ft".to_string(), "ft".to_string()), FT_TO_FT),
    (("ft".to_string(), "yd".to_string()), FT_TO_YD),
    (("ft".to_string(), "mi".to_string()), FT_TO_MI),
    (("ft".to_string(), "nm".to_string()), FT_TO_NM),
    (("ft".to_string(), "mil".to_string()), FT_TO_MIL),
    (("yd".to_string(), "mm".to_string()), YD_TO_MM),
    (("yd".to_string(), "cm".to_string()), YD_TO_CM),
    (("yd".to_string(), "m".to_string()), YD_TO_M),
    (("yd".to_string(), "km".to_string()), YD_TO_KM),
    (("yd".to_string(), "in".to_string()), YD_TO_IN),
    (("yd".to_string(), "ft".to_string()), YD_TO_FT),
    (("yd".to_string(), "yd".to_string()), YD_TO_YD),
    (("yd".to_string(), "mi".to_string()), YD_TO_MI),
    (("yd".to_string(), "nm".to_string()), YD_TO_NM),
    (("yd".to_string(), "mil".to_string()), YD_TO_MIL),
    (("mi".to_string(), "mm".to_string()), MI_TO_MM),
    (("mi".to_string(), "cm".to_string()), MI_TO_CM),
    (("mi".to_string(), "m".to_string()), MI_TO_M),
    (("mi".to_string(), "km".to_string()), MI_TO_KM),
    (("mi".to_string(), "in".to_string()), MI_TO_IN),
    (("mi".to_string(), "ft".to_string()), MI_TO_FT),
    (("mi".to_string(), "yd".to_string()), MI_TO_YD),
    (("mi".to_string(), "mi".to_string()), MI_TO_MI),
    (("mi".to_string(), "nm".to_string()), MI_TO_NM),
    (("mi".to_string(), "mil".to_string()), MI_TO_MIL),
    (("nm".to_string(), "mm".to_string()), NM_TO_MM),
    (("nm".to_string(), "cm".to_string()), NM_TO_CM),
    (("nm".to_string(), "m".to_string()), NM_TO_M),
    (("nm".to_string(), "km".to_string()), NM_TO_KM),
    (("nm".to_string(), "in".to_string()), NM_TO_IN),
    (("nm".to_string(), "ft".to_string()), NM_TO_FT),
    (("nm".to_string(), "yd".to_string()), NM_TO_YD),
    (("nm".to_string(), "mi".to_string()), NM_TO_MI),
    (("nm".to_string(), "nm".to_string()), NM_TO_NM),
    (("nm".to_string(), "mil".to_string()), NM_TO_MIL),
    (("mil".to_string(), "mm".to_string()), MIL_TO_MM),
    (("mil".to_string(), "cm".to_string()), MIL_TO_CM),
    (("mil".to_string(), "m".to_string()), MIL_TO_M),
    (("mil".to_string(), "km".to_string()), MIL_TO_KM),
    (("mil".to_string(), "in".to_string()), MIL_TO_IN),
    (("mil".to_string(), "ft".to_string()), MIL_TO_FT),
    (("mil".to_string(), "yd".to_string()), MIL_TO_YD),
    (("mil".to_string(), "mi".to_string()), MIL_TO_MI),
    (("mil".to_string(), "nm".to_string()), MIL_TO_NM),
    (("mil".to_string(), "mil".to_string()), MIL_TO_MIL),
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
      cm @ "cm" => Self {
        value: 0_f64,
        title: String::from("Centimetres"),
        symbol: String::from(cm)
      },
      m @ "m" => Self {
        value: 0_f64,
        title: String::from("Metres"),
        symbol: String::from(m)
      },
      km @ "km" => Self {
        value: 0_f64,
        title: String::from("Kilometres"),
        symbol: String::from(km)
      },
      inch @ "in" => Self {
        value: 0_f64,
        title: String::from("Inches"),
        symbol: String::from(inch)
      },
      ft @ "ft" => Self {
        value: 0_f64,
        title: String::from("Feet"),
        symbol: String::from(ft)
      },
      yd @ "yd" => Self {
        value: 0_f64,
        title: String::from("Yard"),
        symbol: String::from(yd)
      },
      mi @ "mi" => Self {
        value: 0_f64,
        title: String::from("Miles"),
        symbol: String::from(mi)
      },
      nm @ "nm" => Self {
        value: 0_f64,
        title: String::from("Nautical Miles"),
        symbol: String::from(nm)
      },
      mil @ "mil" => Self {
        value: 0_f64,
        title: String::from("Mils"),
        symbol: String::from(mil)
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

impl LengthUnit {
  pub fn from_mm(x: f64) -> Self {
    Self::with_value(x, "mm")
  }

  pub fn from_cm(x: f64) -> Self {
    Self::with_value(x, "cm")
  }

  pub fn from_m(x: f64) -> Self {
    Self::with_value(x, "m")
  }

  pub fn from_km(x: f64) -> Self {
    Self::with_value(x, "km")
  }

  pub fn from_in(x: f64) -> Self {
    Self::with_value(x, "in")
  }

  pub fn from_ft(x: f64) -> Self {
    Self::with_value(x, "ft")
  }

  pub fn from_yd(x: f64) -> Self {
    Self::with_value(x, "yd")
  }

  pub fn from_mi(x: f64) -> Self {
    Self::with_value(x, "mi")
  }

  pub fn from_nm(x: f64) -> Self {
    Self::with_value(x, "nm")
  }

  pub fn from_mils(x: f64) -> Self {
    Self::with_value(x, "mil")
  }

  pub fn to_mm(self) -> Self {
    self.to(Self::from_symbol("mm"))
  }

  pub fn to_cm(self) -> Self {
    self.to(Self::from_symbol("cm"))
  }

  pub fn to_m(self) -> Self {
    self.to(Self::from_symbol("m"))
  }

  pub fn to_km(self) -> Self {
    self.to(Self::from_symbol("km"))
  }

  pub fn to_in(self) -> Self {
    self.to(Self::from_symbol("in"))
  }

  pub fn to_ft(self) -> Self {
    self.to(Self::from_symbol("ft"))
  }

  pub fn to_yd(self) -> Self {
    self.to(Self::from_symbol("yd"))
  }

  pub fn to_mi(self) -> Self {
    self.to(Self::from_symbol("mi"))
  }

  pub fn to_nm(self) -> Self {
    self.to(Self::from_symbol("nm"))
  }

  pub fn to_mil(self) -> Self {
    self.to(Self::from_symbol("mil"))
  }
}

