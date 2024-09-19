use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("t".to_string(), "t".to_string()), T_TO_T),
    (("t".to_string(), "uk_t".to_string()), T_TO_UKT),
    (("t".to_string(), "us_t".to_string()), T_TO_UST),
    (("t".to_string(), "lb".to_string()), T_TO_LB),
    (("t".to_string(), "oz".to_string()), T_TO_OZ),
    (("t".to_string(), "kg".to_string()), T_TO_KG),
    (("t".to_string(), "g".to_string()), T_TO_G),
    
    (("uk_t".to_string(), "t".to_string()), UKT_TO_T),
    (("uk_t".to_string(), "uk_t".to_string()), UKT_TO_UKT),
    (("uk_t".to_string(), "us_t".to_string()), UKT_TO_UST),
    (("uk_t".to_string(), "lb".to_string()), UKT_TO_LB),
    (("uk_t".to_string(), "oz".to_string()), UKT_TO_OZ),
    (("uk_t".to_string(), "kg".to_string()), UKT_TO_KG),
    (("uk_t".to_string(), "g".to_string()), UKT_TO_G),
    
    (("us_t".to_string(), "t".to_string()), UST_TO_T),
    (("us_t".to_string(), "uk_t".to_string()), UST_TO_UKT),
    (("us_t".to_string(), "us_t".to_string()), UST_TO_UST),
    (("us_t".to_string(), "lb".to_string()), UST_TO_LB),
    (("us_t".to_string(), "oz".to_string()), UST_TO_OZ),
    (("us_t".to_string(), "kg".to_string()), UST_TO_KG),
    (("us_t".to_string(), "g".to_string()), UST_TO_G),
    
    (("lb".to_string(), "t".to_string()), LB_TO_T),
    (("lb".to_string(), "uk_t".to_string()), LB_TO_UKT),
    (("lb".to_string(), "us_t".to_string()), LB_TO_UST),
    (("lb".to_string(), "lb".to_string()), LB_TO_LB),
    (("lb".to_string(), "oz".to_string()), LB_TO_OZ),
    (("lb".to_string(), "kg".to_string()), LB_TO_KG),
    (("lb".to_string(), "g".to_string()), LB_TO_G),
    
    (("oz".to_string(), "t".to_string()), OZ_TO_T),
    (("oz".to_string(), "uk_t".to_string()), OZ_TO_UKT),
    (("oz".to_string(), "us_t".to_string()), OZ_TO_UST),
    (("oz".to_string(), "lb".to_string()), OZ_TO_LB),
    (("oz".to_string(), "oz".to_string()), OZ_TO_OZ),
    (("oz".to_string(), "kg".to_string()), OZ_TO_KG),
    (("oz".to_string(), "g".to_string()), OZ_TO_G),
    
    (("kg".to_string(), "t".to_string()), KG_TO_T),
    (("kg".to_string(), "uk_t".to_string()), KG_TO_UKT),
    (("kg".to_string(), "us_t".to_string()), KG_TO_UST),
    (("kg".to_string(), "lb".to_string()), KG_TO_LB),
    (("kg".to_string(), "oz".to_string()), KG_TO_OZ),
    (("kg".to_string(), "kg".to_string()), KG_TO_KG),
    (("kg".to_string(), "g".to_string()), KG_TO_G),
    
    (("g".to_string(), "t".to_string()), G_TO_T),
    (("g".to_string(), "uk_t".to_string()), G_TO_UKT),
    (("g".to_string(), "us_t".to_string()), G_TO_UST),
    (("g".to_string(), "lb".to_string()), G_TO_LB),
    (("g".to_string(), "oz".to_string()), G_TO_OZ),
    (("g".to_string(), "kg".to_string()), G_TO_KG),
    (("g".to_string(), "g".to_string()), G_TO_G),
    
  ])
});

// #[derive(Hash, Eq, PartialEq, Debug)]
#[derive(Debug)]
pub struct MassUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit for MassUnit {
  fn from_symbol(sym: &str) -> Self {
    match sym {
      t @ "t" => Self {
        value: 0_f64,
        title: String::from("Tonnes"),
        symbol: String::from(t)
      },
      ukt @ "uk_t" => Self {
        value: 0_f64,
        title: String::from("UK tons"),
        symbol: String::from(ukt)
      },
      ust @ "us_t" => Self {
        value: 0_f64,
        title: String::from("US tons"),
        symbol: String::from(ust)
      },
      lb @ "lb" => Self {
        value: 0_f64,
        title: String::from("Pounds"),
        symbol: String::from(lb)
      },
      oz @ "oz" => Self {
        value: 0_f64,
        title: String::from("Ounces"),
        symbol: String::from(oz)
      },
      kg @ "kg" => Self {
        value: 0_f64,
        title: String::from("Kilogrammes"),
        symbol: String::from(kg)
      },
      g @ "g" => Self {
        value: 0_f64,
        title: String::from("Grams"),
        symbol: String::from(g)
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