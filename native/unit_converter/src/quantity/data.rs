use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::QuantityUnit;

mod conversion;
use conversion::*;

static CONVMAP: Lazy<HashMap<(String, String), fn(f64) -> f64>> = Lazy::new(|| {
  HashMap::from([
    (("bit".to_string(), "bit".to_string()), BIT_TO_BIT),
    (("bit".to_string(), "B".to_string()), BIT_TO_B),
    (("bit".to_string(), "KB".to_string()), BIT_TO_KB),
    (("bit".to_string(), "KiB".to_string()), BIT_TO_KIB),
    (("bit".to_string(), "MB".to_string()), BIT_TO_MB),
    (("bit".to_string(), "MiB".to_string()), BIT_TO_MIB),
    (("bit".to_string(), "GB".to_string()), BIT_TO_GB),
    (("bit".to_string(), "GiB".to_string()), BIT_TO_GIB),
    (("bit".to_string(), "TB".to_string()), BIT_TO_TB),
    (("bit".to_string(), "TiB".to_string()), BIT_TO_TIB),

    (("B".to_string(), "bit".to_string()), B_TO_BIT),
    (("B".to_string(), "B".to_string()), B_TO_B),
    (("B".to_string(), "KB".to_string()), B_TO_KB),
    (("B".to_string(), "KiB".to_string()), B_TO_KIB),
    (("B".to_string(), "MB".to_string()), B_TO_MB),
    (("B".to_string(), "MiB".to_string()), B_TO_MIB),
    (("B".to_string(), "GB".to_string()), B_TO_GB),
    (("B".to_string(), "GiB".to_string()), B_TO_GIB),
    (("B".to_string(), "TB".to_string()), B_TO_TB),
    (("B".to_string(), "TiB".to_string()), B_TO_TIB),

    (("KB".to_string(), "bit".to_string()), KB_TO_BIT),
    (("KB".to_string(), "B".to_string()), KB_TO_B),
    (("KB".to_string(), "KB".to_string()), KB_TO_KB),
    (("KB".to_string(), "KiB".to_string()), KB_TO_KIB),
    (("KB".to_string(), "MB".to_string()), KB_TO_MB),
    (("KB".to_string(), "MiB".to_string()), KB_TO_MIB),
    (("KB".to_string(), "GB".to_string()), KB_TO_GB),
    (("KB".to_string(), "GiB".to_string()), KB_TO_GIB),
    (("KB".to_string(), "TB".to_string()), KB_TO_TB),
    (("KB".to_string(), "TiB".to_string()), KB_TO_TIB),

    (("KiB".to_string(), "bit".to_string()), KIB_TO_BIT),
    (("KiB".to_string(), "B".to_string()), KIB_TO_B),
    (("KiB".to_string(), "KB".to_string()), KIB_TO_KB),
    (("KiB".to_string(), "KiB".to_string()), KIB_TO_KIB),
    (("KiB".to_string(), "MB".to_string()), KIB_TO_MB),
    (("KiB".to_string(), "MiB".to_string()), KIB_TO_MIB),
    (("KiB".to_string(), "GB".to_string()), KIB_TO_GB),
    (("KiB".to_string(), "GiB".to_string()), KIB_TO_GIB),
    (("KiB".to_string(), "TB".to_string()), KIB_TO_TB),
    (("KiB".to_string(), "TiB".to_string()), KIB_TO_TIB),

    (("MB".to_string(), "bit".to_string()), MB_TO_BIT),
    (("MB".to_string(), "B".to_string()), MB_TO_B),
    (("MB".to_string(), "KB".to_string()), MB_TO_KB),
    (("MB".to_string(), "KiB".to_string()), MB_TO_KIB),
    (("MB".to_string(), "MB".to_string()), MB_TO_MB),
    (("MB".to_string(), "MiB".to_string()), MB_TO_MIB),
    (("MB".to_string(), "GB".to_string()), MB_TO_GB),
    (("MB".to_string(), "GiB".to_string()), MB_TO_GIB),
    (("MB".to_string(), "TB".to_string()), MB_TO_TB),
    (("MB".to_string(), "TiB".to_string()), MB_TO_TIB),

    (("MiB".to_string(), "bit".to_string()), MIB_TO_BIT),
    (("MiB".to_string(), "B".to_string()), MIB_TO_B),
    (("MiB".to_string(), "KB".to_string()), MIB_TO_KB),
    (("MiB".to_string(), "KiB".to_string()), MIB_TO_KIB),
    (("MiB".to_string(), "MB".to_string()), MIB_TO_MB),
    (("MiB".to_string(), "MiB".to_string()), MIB_TO_MIB),
    (("MiB".to_string(), "GB".to_string()), MIB_TO_GB),
    (("MiB".to_string(), "GiB".to_string()), MIB_TO_GIB),
    (("MiB".to_string(), "TB".to_string()), MIB_TO_TB),
    (("MiB".to_string(), "TiB".to_string()), MIB_TO_TIB),

    (("GB".to_string(), "bit".to_string()), GB_TO_BIT),
    (("GB".to_string(), "B".to_string()), GB_TO_B),
    (("GB".to_string(), "KB".to_string()), GB_TO_KB),
    (("GB".to_string(), "KiB".to_string()), GB_TO_KIB),
    (("GB".to_string(), "MB".to_string()), GB_TO_MB),
    (("GB".to_string(), "MiB".to_string()), GB_TO_MIB),
    (("GB".to_string(), "GB".to_string()), GB_TO_GB),
    (("GB".to_string(), "GiB".to_string()), GB_TO_GIB),
    (("GB".to_string(), "TB".to_string()), GB_TO_TB),
    (("GB".to_string(), "TiB".to_string()), GB_TO_TIB),

    (("GiB".to_string(), "bit".to_string()), GIB_TO_BIT),
    (("GiB".to_string(), "B".to_string()), GIB_TO_B),
    (("GiB".to_string(), "KB".to_string()), GIB_TO_KB),
    (("GiB".to_string(), "KiB".to_string()), GIB_TO_KIB),
    (("GiB".to_string(), "MB".to_string()), GIB_TO_MB),
    (("GiB".to_string(), "MiB".to_string()), GIB_TO_MIB),
    (("GiB".to_string(), "GB".to_string()), GIB_TO_GB),
    (("GiB".to_string(), "GiB".to_string()), GIB_TO_GIB),
    (("GiB".to_string(), "TB".to_string()), GIB_TO_TB),
    (("GiB".to_string(), "TiB".to_string()), GIB_TO_TIB),

    (("TB".to_string(), "bit".to_string()), TB_TO_BIT),
    (("TB".to_string(), "B".to_string()), TB_TO_B),
    (("TB".to_string(), "KB".to_string()), TB_TO_KB),
    (("TB".to_string(), "KiB".to_string()), TB_TO_KIB),
    (("TB".to_string(), "MB".to_string()), TB_TO_MB),
    (("TB".to_string(), "MiB".to_string()), TB_TO_MIB),
    (("TB".to_string(), "GB".to_string()), TB_TO_GB),
    (("TB".to_string(), "GiB".to_string()), TB_TO_GIB),
    (("TB".to_string(), "TB".to_string()), TB_TO_TB),
    (("TB".to_string(), "TiB".to_string()), TB_TO_TIB),

    (("TiB".to_string(), "bit".to_string()), TIB_TO_BIT),
    (("TiB".to_string(), "B".to_string()), TIB_TO_B),
    (("TiB".to_string(), "KB".to_string()), TIB_TO_KB),
    (("TiB".to_string(), "KiB".to_string()), TIB_TO_KIB),
    (("TiB".to_string(), "MB".to_string()), TIB_TO_MB),
    (("TiB".to_string(), "MiB".to_string()), TIB_TO_MIB),
    (("TiB".to_string(), "GB".to_string()), TIB_TO_GB),
    (("TiB".to_string(), "GiB".to_string()), TIB_TO_GIB),
    (("TiB".to_string(), "TB".to_string()), TIB_TO_TB),
    (("TiB".to_string(), "TiB".to_string()), TIB_TO_TIB),
  ])
});

#[derive(Debug)]
pub struct DataUnit {
  value: f64,
  title: String,
  symbol: String
}

impl QuantityUnit for DataUnit {
  fn from_symbol(sym: &str) -> Self {
    match sym {
      bit @ "bit" => Self {
        value: 0_f64,
        title: String::from("Bits"),
        symbol: String::from(bit)
      },
      b @ "B" => Self {
        value: 0_f64,
        title: String::from("Bytes"),
        symbol: String::from(b)
      },
      kb @ "KB" => Self {
        value: 0_f64,
        title: String::from("Kilobytes"),
        symbol: String::from(kb)
      },
      kib @ "KiB" => Self {
        value: 0_f64,
        title: String::from("Kibibytes"),
        symbol: String::from(kib)
      },
      mb @ "MB" => Self {
        value: 0_f64,
        title: String::from("Megabytes"),
        symbol: String::from(mb)
      },
      mib @ "MiB" => Self {
        value: 0_f64,
        title: String::from("Mebibytes"),
        symbol: String::from(mib)
      },
      gb @ "GB" => Self {
        value: 0_f64,
        title: String::from("Gigabytes"),
        symbol: String::from(gb)
      },
      gib @ "GiB" => Self {
        value: 0_f64,
        title: String::from("Gibibytes"),
        symbol: String::from(gib)
      },
      tb @ "TB" => Self {
        value: 0_f64,
        title: String::from("Terabytes"),
        symbol: String::from(tb)
      },
      tib @ "TiB" => Self {
        value: 0_f64,
        title: String::from("Tebibytes"),
        symbol: String::from(tib)
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