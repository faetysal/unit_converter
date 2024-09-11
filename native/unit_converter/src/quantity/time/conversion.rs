pub const SECS_TO_MINS: fn(f64) -> f64 = |x| {
  x / 60_f64
};

pub const SECS_TO_HOURS: fn(f64) -> f64 = |x| {
  x / (60 * 60) as f64
};

pub const SECS_TO_DAYS: fn(f64) -> f64 = |x| {
  x / (60 * 60 * 24) as f64
};