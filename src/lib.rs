#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod animal;

#[napi]
fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
fn is_even(n: i32) -> bool {
  n % 2 == 0
}
