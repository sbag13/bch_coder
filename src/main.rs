#![feature(vec_resize_default)]
extern crate bitvec;
extern crate env_logger;
extern crate itertools;
extern crate log;
extern crate rand;

// const POWER: usize = 9;
// const TWO_TO_POWER: usize = 512;

mod bch_bitvec;
mod berlekamp_decoder;
mod common;
mod decoder;
mod encoder;
mod simple_decoder;
mod tests;
mod mycoder_tests;

fn main() {}

//TODO
// add comments to doc, maybe

// Pytania
// czy zwracać errory
