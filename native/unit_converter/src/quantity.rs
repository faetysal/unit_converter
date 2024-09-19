// use std::ops::{Add, Sub, Mul, Div};
// use std::marker::Copy;

pub mod time;
pub mod length;
pub mod area;
pub mod temperature;
pub mod volume;

// trait Number: Copy + Add + Sub + Mul + Div {}

pub trait QuantityUnit {
  fn from_symbol(sym: &str) -> Self;

  fn with_value(value: f64, sym: &str) -> Self;

  fn to(self, other: Self) -> Self;

  fn value(&self) -> f64;

  fn title(&self) -> &str;

  fn symbol(&self) -> &str;
}