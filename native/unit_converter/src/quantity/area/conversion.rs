pub const AC_TO_AC: fn(f64) -> f64 = |x| x;
pub const AC_TO_A: fn(f64) -> f64 = |x| x * 40.468564224;
pub const AC_TO_HA: fn(f64) -> f64 = |x| x / 2.4710538149;
pub const AC_TO_CM2: fn(f64) -> f64 = |x| x * 40468564.224;
pub const AC_TO_FT2: fn(f64) -> f64 = |x| x * 43560_f64;
pub const AC_TO_IN2: fn(f64) -> f64 = |x| x * 6272640_f64;
pub const AC_TO_M2: fn(f64) -> f64 = |x| x * 4046.8564224;

pub const A_TO_AC: fn(f64) -> f64 = |x| x / 40.4685643005;
pub const A_TO_A: fn(f64) -> f64 = |x| x;
pub const A_TO_HA: fn(f64) -> f64 = |x| x / 100_f64;
pub const A_TO_CM2: fn(f64) -> f64 = |x| x * 1e6;
pub const A_TO_FT2: fn(f64) -> f64 = |x| x * 1076.391041671;
pub const A_TO_IN2: fn(f64) -> f64 = |x| x * 155000.31000062;
pub const A_TO_M2: fn(f64) -> f64 = |x| x * 100_f64;

pub const HA_TO_AC: fn(f64) -> f64 = |x| x * 2.4710538147;
pub const HA_TO_A: fn(f64) -> f64 = |x| x * 100_f64;
pub const HA_TO_HA: fn(f64) -> f64 = |x| x;
pub const HA_TO_CM2: fn(f64) -> f64 = |x| x * 1e8;
pub const HA_TO_FT2: fn(f64) -> f64 = |x| x * 107639.1041671;
pub const HA_TO_IN2: fn(f64) -> f64 = |x| x * 15500031.000062;
pub const HA_TO_M2: fn(f64) -> f64 = |x| x * 10000_f64;

pub const CM2_TO_AC: fn(f64) -> f64 = |x| x * 2.47105381e-8;
pub const CM2_TO_A: fn(f64) -> f64 = |x| x / 1e6;
pub const CM2_TO_HA: fn(f64) -> f64 = |x| x / 1e8;
pub const CM2_TO_CM2: fn(f64) -> f64 = |x| x;
pub const CM2_TO_FT2: fn(f64) -> f64 = |x| x / 929.0304359661;
pub const CM2_TO_IN2: fn(f64) -> f64 = |x| x / 6.4516;
pub const CM2_TO_M2: fn(f64) -> f64 = |x| x / 10000_f64;

pub const FT2_TO_AC: fn(f64) -> f64 = |x| x / 43560.07805966;
pub const FT2_TO_A: fn(f64) -> f64 = |x| x / 1076.391041671;
pub const FT2_TO_HA: fn(f64) -> f64 = |x| x / 107639.15051182;
pub const FT2_TO_CM2: fn(f64) -> f64 = |x| x * 929.0304;
pub const FT2_TO_FT2: fn(f64) -> f64 = |x| x;
pub const FT2_TO_IN2: fn(f64) -> f64 = |x| x * 144_f64;
pub const FT2_TO_M2: fn(f64) -> f64 = |x| x / 10.7639104167;

pub const IN2_TO_AC: fn(f64) -> f64 = |x| x * 1.59422508e-7;
pub const IN2_TO_A: fn(f64) -> f64 = |x| x / 155000.31000062;
pub const IN2_TO_HA: fn(f64) -> f64 = |x| x * 6.45160000e-8;
pub const IN2_TO_CM2: fn(f64) -> f64 = |x| x * 6.4516;
pub const IN2_TO_FT2: fn(f64) -> f64 = |x| x / 144.0000009216;
pub const IN2_TO_IN2: fn(f64) -> f64 = |x| x;
pub const IN2_TO_M2: fn(f64) -> f64 = |x| x / 1550.0031000062;

pub const M2_TO_AC: fn(f64) -> f64 = |x| x / 4046.8561188869;
pub const M2_TO_A: fn(f64) -> f64 = |x| x / 100_f64;
pub const M2_TO_HA: fn(f64) -> f64 = |x| x / 10000_f64;
pub const M2_TO_CM2: fn(f64) -> f64 = |x| x * 10000_f64;
pub const M2_TO_FT2: fn(f64) -> f64 = |x| x * 10.7639104167;
pub const M2_TO_IN2: fn(f64) -> f64 = |x| x * 1550.0031000062;
pub const M2_TO_M2: fn(f64) -> f64 = |x| x;
