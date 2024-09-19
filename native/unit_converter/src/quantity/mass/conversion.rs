pub const T_TO_T: fn(f64) -> f64 = |x| x;
pub const T_TO_UKT: fn(f64) -> f64 = |x| x / 1.0160469088;
pub const T_TO_UST: fn(f64) -> f64 = |x| x * 1.1023113109;
pub const T_TO_LB: fn(f64) -> f64 = |x| x * 2204.6226218488;
pub const T_TO_OZ: fn(f64) -> f64 = |x| x * 35273.96194958;
pub const T_TO_KG: fn(f64) -> f64 = |x| x * 1000_f64;
pub const T_TO_G: fn(f64) -> f64 = |x| x * 1e6;

pub const UKT_TO_T: fn(f64) -> f64 = |x| x * 1.0160469088;
pub const UKT_TO_UKT: fn(f64) -> f64 = |x| x;
pub const UKT_TO_UST: fn(f64) -> f64 = |x| x * 1.12;
pub const UKT_TO_LB: fn(f64) -> f64 = |x| x * 2240_f64;
pub const UKT_TO_OZ: fn(f64) -> f64 = |x| x * 35840_f64;
pub const UKT_TO_KG: fn(f64) -> f64 = |x| x * 1016.0469088;
pub const UKT_TO_G: fn(f64) -> f64 = |x| x * 1016046.9088;

pub const UST_TO_T: fn(f64) -> f64 = |x| x / 1.1023113109;
pub const UST_TO_UKT: fn(f64) -> f64 = |x| x / 1.1199999999;
pub const UST_TO_UST: fn(f64) -> f64 = |x| x;
pub const UST_TO_LB: fn(f64) -> f64 = |x| x * 2000_f64;
pub const UST_TO_OZ: fn(f64) -> f64 = |x| x * 32000_f64;
pub const UST_TO_KG: fn(f64) -> f64 = |x| x * 907.18474;
pub const UST_TO_G: fn(f64) -> f64 = |x| x * 907184.74;

pub const LB_TO_T: fn(f64) -> f64 = |x| x / 2204.622476038;
pub const LB_TO_UKT: fn(f64) -> f64 = |x| x / 2239.99985664;
pub const LB_TO_UST: fn(f64) -> f64 = |x| x / 2000_f64;
pub const LB_TO_LB: fn(f64) -> f64 = |x| x;
pub const LB_TO_OZ: fn(f64) -> f64 = |x| x * 16_f64;
pub const LB_TO_KG: fn(f64) -> f64 = |x| x / 2.2046226218;
pub const LB_TO_G: fn(f64) -> f64 = |x| x * 453.59237;

pub const OZ_TO_T: fn(f64) -> f64 = |x| x / 35273.99072294;
pub const OZ_TO_UKT: fn(f64) -> f64 = |x| x / 35839.981649929;
pub const OZ_TO_UST: fn(f64) -> f64 = |x| x / 32000_f64;
pub const OZ_TO_LB: fn(f64) -> f64 = |x| x / 16_f64;
pub const OZ_TO_OZ: fn(f64) -> f64 = |x| x;
pub const OZ_TO_KG: fn(f64) -> f64 = |x| x / 35.2739619807;
pub const OZ_TO_G: fn(f64) -> f64 = |x| x * 28.349523125;

pub const KG_TO_T: fn(f64) -> f64 = |x| x / 1000_f64;
pub const KG_TO_UKT: fn(f64) -> f64 = |x| x / 1016.0469373043;
pub const KG_TO_UST: fn(f64) -> f64 = |x| x / 907.1847489906;
pub const KG_TO_LB: fn(f64) -> f64 = |x| x * 2.2046226218;
pub const KG_TO_OZ: fn(f64) -> f64 = |x| x * 35.273919496;
pub const KG_TO_KG: fn(f64) -> f64 = |x| x;
pub const KG_TO_G: fn(f64) -> f64 = |x| x * 1000_f64;

pub const G_TO_T: fn(f64) -> f64 = |x| x / 1e6;
pub const G_TO_UKT: fn(f64) -> f64 = |x| x * 9.84206528e-7;
pub const G_TO_UST: fn(f64) -> f64 = |x| x / 907194.04880704;
pub const G_TO_LB: fn(f64) -> f64 = |x| x / 453.5923744953;
pub const G_TO_OZ: fn(f64) -> f64 = |x| x / 28.3495231648;
pub const G_TO_KG: fn(f64) -> f64 = |x| x / 1000_f64;
pub const G_TO_G: fn(f64) -> f64 = |x| x;