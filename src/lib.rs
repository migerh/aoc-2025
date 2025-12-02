#![allow(clippy::unit_arg)]

use aoc_runner_derive::aoc_lib;
#[macro_use]
extern crate lazy_static;
extern crate rayon;

pub mod day01;
pub mod day02;
pub mod utils;

aoc_lib! { year = 2025 }
