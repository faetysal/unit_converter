pub const MILLIS_TO_MILLIS: fn(f64) -> f64 = |x| x;
pub const MILLIS_TO_SECS: fn(f64) -> f64 = |x| x / 1000_f64;
pub const MILLIS_TO_MINS: fn(f64) -> f64 = |x| x / (60 * 1000) as f64;
pub const MILLIS_TO_HOURS: fn(f64) -> f64 = |x| x / (60 * 60 * 1000) as f64;
pub const MILLIS_TO_DAYS: fn(f64) -> f64 = |x| x / (60 * 60 * 24 * 1000) as f64;

pub const SECS_TO_MILLIS: fn(f64) -> f64 = |x| x * 1000_f64;
pub const SECS_TO_SECS: fn(f64) -> f64 = |x| x;
pub const SECS_TO_MINS: fn(f64) -> f64 = |x| x / 60_f64;
pub const SECS_TO_HOURS: fn(f64) -> f64 = |x| x / (60 * 60) as f64;
pub const SECS_TO_DAYS: fn(f64) -> f64 = |x| x / (60 * 60 * 24) as f64;

pub const MINS_TO_MILLIS: fn(f64) -> f64 = |x| x * (60 * 1000) as f64;
pub const MINS_TO_SECS: fn(f64) -> f64 = |x| x * 60_f64;
pub const MINS_TO_MINS: fn(f64) -> f64 = |x| x;
pub const MINS_TO_HOURS: fn(f64) -> f64 = |x| x / 60_f64;
pub const MINS_TO_DAYS: fn(f64) -> f64 = |x| x / (60 * 24) as f64;

pub const HOURS_TO_MILLIS: fn(f64) -> f64 = |x| x * (60 * 60 * 1000) as f64;
pub const HOURS_TO_SECS: fn(f64) -> f64 = |x| x * (60 * 60) as f64;
pub const HOURS_TO_MINS: fn(f64) -> f64 = |x| x * 60_f64;
pub const HOURS_TO_HOURS: fn(f64) -> f64 = |x| x;
pub const HOURS_TO_DAYS: fn(f64) -> f64 = |x| x / 24_f64;

pub const DAYS_TO_MILLIS: fn(f64) -> f64 = |x| x * (60 * 60 * 24 * 1000) as f64;
pub const DAYS_TO_SECS: fn(f64) -> f64 = |x| x * (60 * 60 * 24) as f64;
pub const DAYS_TO_MINS: fn(f64) -> f64 = |x| x * (60 * 24) as f64;
pub const DAYS_TO_HOURS: fn(f64) -> f64 = |x| x * 24_f64;
pub const DAYS_TO_DAYS: fn(f64) -> f64 = |x| x;