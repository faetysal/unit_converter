pub const UKGAL_TO_UKGAL: fn(f64) -> f64 = |x| x;
pub const UKGAL_TO_USGAL: fn(f64) -> f64 = |x| x * 1.2009499255;
pub const UKGAL_TO_L: fn(f64) -> f64 = |x| x * 4.54609;
pub const UKGAL_TO_ML: fn(f64) -> f64 = |x| x * 4546.09;
pub const UKGAL_TO_CM3: fn(f64) -> f64 = |x| x * 4546.09;
pub const UKGAL_TO_M3: fn(f64) -> f64 = |x| x / 219.9692482991;
pub const UKGAL_TO_IN3: fn(f64) -> f64 = |x| x * 277.419432791;
pub const UKGAL_TO_FT3: fn(f64) -> f64 = |x| x / 6.2288354604;

pub const USGAL_TO_UKGAL: fn(f64) -> f64 = |x| x / 1.2009499255;
pub const USGAL_TO_USGAL: fn(f64) -> f64 = |x| x;
pub const USGAL_TO_L: fn(f64) -> f64 = |x| x * 3.785411784;
pub const USGAL_TO_ML: fn(f64) -> f64 = |x| x * 3785.411784;
pub const USGAL_TO_CM3: fn(f64) -> f64 = |x| x * 3785.411784;
pub const USGAL_TO_M3: fn(f64) -> f64 = |x| x / 264.1720512416;
pub const USGAL_TO_IN3: fn(f64) -> f64 = |x| x * 231_f64;
pub const USGAL_TO_FT3: fn(f64) -> f64 = |x| x / 7.480519478;

pub const L_TO_UKGAL: fn(f64) -> f64 = |x| x / 4.54609;
pub const L_TO_USGAL: fn(f64) -> f64 = |x| x / 3.7854117834;
pub const L_TO_L: fn(f64) -> f64 = |x| x;
pub const L_TO_ML: fn(f64) -> f64 = |x| x * 1000_f64;
pub const L_TO_CM3: fn(f64) -> f64 = |x| x * 1000_f64;
pub const L_TO_M3: fn(f64) -> f64 = |x| x / 1000_f64;
pub const L_TO_IN3: fn(f64) -> f64 = |x| x * 61.0237440947;
pub const L_TO_FT3: fn(f64) -> f64 = |x| x / 28.3168466092;

pub const ML_TO_UKGAL: fn(f64) -> f64 = |x| x / 4546.0909981943;
pub const ML_TO_USGAL: fn(f64) -> f64 = |x| x / 3875.4111013237;
pub const ML_TO_L: fn(f64) -> f64 = |x| x / 1000_f64;
pub const ML_TO_ML: fn(f64) -> f64 = |x| x;
pub const ML_TO_CM3: fn(f64) -> f64 = |x| x * 1_f64;
pub const ML_TO_M3: fn(f64) -> f64 = |x| x / 1e6;
pub const ML_TO_IN3: fn(f64) -> f64 = |x| x / 16.3870639986;
pub const ML_TO_FT3: fn(f64) -> f64 = |x| x / 28316.819907857;

pub const CM3_TO_UKGAL: fn(f64) -> f64 = |x| x / 4546.0909981943;
pub const CM3_TO_USGAL: fn(f64) -> f64 = |x| x / 3785.4111013237;
pub const CM3_TO_L: fn(f64) -> f64 = |x| x / 1000_f64;
pub const CM3_TO_ML: fn(f64) -> f64 = |x| x * 1_f64;
pub const CM3_TO_CM3: fn(f64) -> f64 = |x| x;
pub const CM3_TO_M3: fn(f64) -> f64 = |x| x / 1e6;
pub const CM3_TO_IN3: fn(f64) -> f64 = |x| x / 16.3870639986;
pub const CM3_TO_FT3: fn(f64) -> f64 = |x| x / 28316.819907857;

pub const M3_TO_UKGAL: fn(f64) -> f64 = |x| x * 219.9692482991;
pub const M3_TO_USGAL: fn(f64) -> f64 = |x| x * 264.1720523581;
pub const M3_TO_L: fn(f64) -> f64 = |x| x * 1000_f64;
pub const M3_TO_ML: fn(f64) -> f64 = |x| x * 1e6;
pub const M3_TO_CM3: fn(f64) -> f64 = |x| x * 1e6;
pub const M3_TO_M3: fn(f64) -> f64 = |x| x;
pub const M3_TO_IN3: fn(f64) -> f64 = |x| x * 61023.744094732;
pub const M3_TO_FT3: fn(f64) -> f64 = |x| x * 35.3146667215;

pub const IN3_TO_UKGAL: fn(f64) -> f64 = |x| x / 277.4194366327;
pub const IN3_TO_USGAL: fn(f64) -> f64 = |x| x / 231.0000015477;
pub const IN3_TO_L: fn(f64) -> f64 = |x| x / 61.0237440947;
pub const IN3_TO_ML: fn(f64) -> f64 = |x| x * 16.387064;
pub const IN3_TO_CM3: fn(f64) -> f64 = |x| x * 16.387064;
pub const IN3_TO_M3: fn(f64) -> f64 = |x| x / 61023.610034722;
pub const IN3_TO_IN3: fn(f64) -> f64 = |x| x;
pub const IN3_TO_FT3: fn(f64) -> f64 = |x| x / 1728.0000110592;

pub const FT3_TO_UKGAL: fn(f64) -> f64 = |x| x * 6.228835459;
pub const FT3_TO_USGAL: fn(f64) -> f64 = |x| x * 7.4805194805;
pub const FT3_TO_L: fn(f64) -> f64 = |x| x * 28.316846592;
pub const FT3_TO_ML: fn(f64) -> f64 = |x| x * 28316.846592;
pub const FT3_TO_CM3: fn(f64) -> f64 = |x| x * 28316.846592;
pub const FT3_TO_M3: fn(f64) -> f64 = |x| x / 35.3146667115;
pub const FT3_TO_IN3: fn(f64) -> f64 = |x| x * 1728_f64;
pub const FT3_TO_FT3: fn(f64) -> f64 = |x| x;