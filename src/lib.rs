#![deny(clippy::all)]

mod algo;

use napi_derive::napi;

#[napi]
pub fn plus_200(input: u32) -> u32 {
  input + 200
}
