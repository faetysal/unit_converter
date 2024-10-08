pub const MM_TO_MM: fn(f64) -> f64 = |x| x;
pub const MM_TO_CM: fn(f64) -> f64 = |x| x / 10_f64;
pub const MM_TO_M: fn(f64) -> f64 = |x| x / 1000_f64;
pub const MM_TO_KM: fn(f64) -> f64 = |x| x / 10_f64.powi(6);
pub const MM_TO_IN: fn(f64) -> f64 = |x| x / 25.4;
pub const MM_TO_FT: fn(f64) -> f64 = |x| x / 304.8;
pub const MM_TO_YD: fn(f64) -> f64 = |x| x / 914.4;
pub const MM_TO_MI: fn(f64) -> f64 = |x| x / (1.609344 * 10_f64.powi(6));
pub const MM_TO_NM: fn(f64) -> f64 = |x| x / (1.852 * 10_f64.powi(6));
pub const MM_TO_MIL: fn(f64) -> f64 = |x| x * 39.37;

pub const CM_TO_MM: fn(f64) -> f64 = |x| x * 10_f64;
pub const CM_TO_CM: fn(f64) -> f64 = |x| x;
pub const CM_TO_M: fn(f64) -> f64 = |x| x / 100_f64;
pub const CM_TO_KM: fn(f64) -> f64 = |x| x * 10_f64.powi(5);
pub const CM_TO_IN: fn(f64) -> f64 = |x| x / 2.54;
pub const CM_TO_FT: fn(f64) -> f64 = |x| x / 30.48;
pub const CM_TO_YD: fn(f64) -> f64 = |x| x / 91.44;
pub const CM_TO_MI: fn(f64) -> f64 = |x| x / 160934.4;
pub const CM_TO_NM: fn(f64) -> f64 = |x| x / 185200_f64;
pub const CM_TO_MIL: fn(f64) -> f64 = |x| x * 393.7007874016;

pub const M_TO_MM: fn(f64) -> f64 = |x| x * 1000_f64;
pub const M_TO_CM: fn(f64) -> f64 = |x| x * 100_f64;
pub const M_TO_M: fn(f64) -> f64 = |x| x;
pub const M_TO_KM: fn(f64) -> f64 = |x| x / 1000_f64;
pub const M_TO_IN: fn(f64) -> f64 = |x| x * 39.37;
pub const M_TO_FT: fn(f64) -> f64 = |x| x * 3.280839895;
pub const M_TO_YD: fn(f64) -> f64 = |x| x * 1.0936132983;
pub const M_TO_MI: fn(f64) -> f64 = |x| x / 1609.344;
pub const M_TO_NM: fn(f64) -> f64 = |x| x / 1852_f64;
pub const M_TO_MIL: fn(f64) -> f64 = |x| x * 39370.078740;

pub const KM_TO_MM: fn(f64) -> f64 = |x| x * 10_f64.powi(6);
pub const KM_TO_CM: fn(f64) -> f64 = |x| x * 10_f64.powi(5);
pub const KM_TO_M: fn(f64) -> f64 = |x| x * 1000_f64;
pub const KM_TO_KM: fn(f64) -> f64 = |x| x;
pub const KM_TO_IN: fn(f64) -> f64 = |x| x * 39370.078740157;
pub const KM_TO_FT: fn(f64) -> f64 = |x| x * 3280.8398950131;
pub const KM_TO_YD: fn(f64) -> f64 = |x| x * 1093.6132983377;
pub const KM_TO_MI: fn(f64) -> f64 = |x| x / 1.609344;
pub const KM_TO_NM: fn(f64) -> f64 = |x| x / 1.852;
pub const KM_TO_MIL: fn(f64) -> f64 = |x| x * (3.937 * 10_f64.powi(7));

pub const IN_TO_MM: fn(f64) -> f64 = |x| x * 25.4;
pub const IN_TO_CM: fn(f64) -> f64 = |x| x * 2.54;
pub const IN_TO_M: fn(f64) -> f64 = |x| x / 39.37;
pub const IN_TO_KM: fn(f64) -> f64 = |x| x / 39370.1;
pub const IN_TO_IN: fn(f64) -> f64 = |x| x;
pub const IN_TO_FT: fn(f64) -> f64 = |x| x / 12_f64;
pub const IN_TO_YD: fn(f64) -> f64 = |x| x / 36_f64;
pub const IN_TO_MI: fn(f64) -> f64 = |x| x / 63360_f64;
pub const IN_TO_NM: fn(f64) -> f64 = |x| x / 72913.400753925;
pub const IN_TO_MIL: fn(f64) -> f64 = |x| x * 1000_f64;

pub const FT_TO_MM: fn(f64) -> f64 = |x| x * 304.8;
pub const FT_TO_CM: fn(f64) -> f64 = |x| x * 30.48;
pub const FT_TO_M: fn(f64) -> f64 = |x| x * 0.3048;
pub const FT_TO_KM: fn(f64) -> f64 = |x| x / 3280.84;
pub const FT_TO_IN: fn(f64) -> f64 = |x| x * 12_f64;
pub const FT_TO_FT: fn(f64) -> f64 = |x| x;
pub const FT_TO_YD: fn(f64) -> f64 = |x| x / 3_f64;
pub const FT_TO_MI: fn(f64) -> f64 = |x| x / 5280_f64;
pub const FT_TO_NM: fn(f64) -> f64 = |x| x / 6076.1167294937;
pub const FT_TO_MIL: fn(f64) -> f64 = |x| x * 12000_f64;

pub const YD_TO_MM: fn(f64) -> f64 = |x| x * 914.4;
pub const YD_TO_CM: fn(f64) -> f64 = |x| x * 91.44;
pub const YD_TO_M: fn(f64) -> f64 = |x| x / 1.0936132983;
pub const YD_TO_KM: fn(f64) -> f64 = |x| x / 1093.6132983377;
pub const YD_TO_IN: fn(f64) -> f64 = |x| x * 36_f64;
pub const YD_TO_FT: fn(f64) -> f64 = |x| x * 3_f64;
pub const YD_TO_YD: fn(f64) -> f64 = |x| x;
pub const YD_TO_MI: fn(f64) -> f64 = |x| x / 1760_f64;
pub const YD_TO_NM: fn(f64) -> f64 = |x| x / 2025.3718329514;
pub const YD_TO_MIL: fn(f64) -> f64 = |x| x * 36000_f64;

pub const MI_TO_MM: fn(f64) -> f64 = |x| x * 1609344_f64;
pub const MI_TO_CM: fn(f64) -> f64 = |x| x * 160934.4;
pub const MI_TO_M: fn(f64) -> f64 = |x| x * 1609.344;
pub const MI_TO_KM: fn(f64) -> f64 = |x| x * 1.609344;
pub const MI_TO_IN: fn(f64) -> f64 = |x| x * 63360_f64;
pub const MI_TO_FT: fn(f64) -> f64 = |x| x * 5280_f64;
pub const MI_TO_YD: fn(f64) -> f64 = |x| x * 1760_f64;
pub const MI_TO_MI: fn(f64) -> f64 = |x| x;
pub const MI_TO_NM: fn(f64) -> f64 = |x| x / 1.150779448;
pub const MI_TO_MIL: fn(f64) -> f64 = |x| x * (6.336 * 10_f64.powi(7));

pub const NM_TO_MM: fn(f64) -> f64 = |x| x * (1.852 * 10_f64.powi(6));
pub const NM_TO_CM: fn(f64) -> f64 = |x| x * 185200_f64;
pub const NM_TO_M: fn(f64) -> f64 = |x| x * 1852_f64;
pub const NM_TO_KM: fn(f64) -> f64 = |x| x * 1.852;
pub const NM_TO_IN: fn(f64) -> f64 = |x| x * 72913.385826772;
pub const NM_TO_FT: fn(f64) -> f64 = |x| x * 6076.1154855643;
pub const NM_TO_YD: fn(f64) -> f64 = |x| x * 2025.3718285214;
pub const NM_TO_MI: fn(f64) -> f64 = |x| x * 1.150779448;
pub const NM_TO_NM: fn(f64) -> f64 = |x| x;
pub const NM_TO_MIL: fn(f64) -> f64 = |x| x * 72913385.826772;

pub const MIL_TO_MM: fn(f64) -> f64 = |x| x / 39.37;
pub const MIL_TO_CM: fn(f64) -> f64 = |x| x / 393.7007874016;
pub const MIL_TO_M: fn(f64) -> f64 = |x| x / 39370.078740157;
pub const MIL_TO_KM: fn(f64) -> f64 = |x| x * (3.937 * 10_f64.powi(7));
pub const MIL_TO_IN: fn(f64) -> f64 = |x| x / 1000_f64;
pub const MIL_TO_FT: fn(f64) -> f64 = |x| x / 12000_f64;
pub const MIL_TO_YD: fn(f64) -> f64 = |x| x / 36000_f64;
pub const MIL_TO_MI: fn(f64) -> f64 = |x| x / (6.336 * 10_f64.powi(7));
pub const MIL_TO_NM: fn(f64) -> f64 = |x| x / 1.37149028 * 10_f64.powi(-8);
pub const MIL_TO_MIL: fn(f64) -> f64 = |x| x;
