#![feature(portable_simd)]
extern crate core;


mod day_02;


// change this use for different days
use day_02::*;

fn main() {
    println!("first part: {:?}", first_part());
    println!("second part: {:?}", second_part());
}
