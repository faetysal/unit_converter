// use std::ops::{Add, Sub, Mul, Div};
// use std::marker::Copy;

pub mod time;

// trait Number: Copy + Add + Sub + Mul + Div {}

pub trait QuantityUnit {
  fn from_symbol(value: f64, sym: &str) -> Self;

  fn value(&self) -> f64;

  fn title(&self) -> &str;

  fn symbol(&self) -> &str;
}