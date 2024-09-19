pub const C_TO_C: fn(f64) -> f64 = |x| x;
pub const C_TO_F: fn(f64) -> f64 = |x| convert(x, CELCIUS_SCALE, FAHRENHEIT_SCALE);
pub const C_TO_K: fn(f64) -> f64 = |x| convert(x, CELCIUS_SCALE, KELVIN_SCALE);

pub const F_TO_C: fn(f64) -> f64 = |x| convert(x, FAHRENHEIT_SCALE, CELCIUS_SCALE);
pub const F_TO_F: fn(f64) -> f64 = |x| x;
pub const F_TO_K: fn(f64) -> f64 = |x| convert(x, FAHRENHEIT_SCALE, KELVIN_SCALE);

pub const K_TO_C: fn(f64) -> f64 = |x| convert(x, KELVIN_SCALE, CELCIUS_SCALE);
pub const K_TO_F: fn(f64) -> f64 = |x| convert(x, KELVIN_SCALE, FAHRENHEIT_SCALE);
pub const K_TO_K: fn(f64) -> f64 = |x| x;

const CELCIUS_SCALE: (f64, f64) = (0_f64, 100_f64);
const FAHRENHEIT_SCALE: (f64, f64) = (32_f64, 212_f64);
const KELVIN_SCALE: (f64, f64) = (273.15, 373.15);

fn convert(x: f64, from_scale: (f64, f64), to_scale: (f64, f64)) -> f64 {
  let (x0, x1) = from_scale;
  let (y0, y1) = to_scale;

  let result = ( ((x - x0) * (y1 - y0)) / (x1 - x0) ) + y0;
  result
}