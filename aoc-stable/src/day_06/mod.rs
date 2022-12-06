use std::borrow::Borrow;
use std::collections::HashSet;
use std::iter::{Enumerate, Map};
use std::ops::{BitOr, BitOrAssign, Shl};
use std::str::{Chars, FromStr};

#[inline]
pub fn first_part() -> usize {
    const START_SEQUENCE_AMOUNT: usize = 4;
    let mut buffer = [0; START_SEQUENCE_AMOUNT];

    for (i, x) in include_str!("input.txt")
        .chars()
        .map(|c| c as u32 - 96)
        .enumerate() {

        buffer[i % START_SEQUENCE_AMOUNT] = x;
        if i >= START_SEQUENCE_AMOUNT {
            let bin = buffer
                .iter()
                .fold(0, |acc, x| { acc | 1u32.shl(*x) });

            if count_set_bits(bin) == START_SEQUENCE_AMOUNT as u32 {
                return i + 1;
            }
        }
    }

    panic!()
}

fn count_set_bits(x: u32) -> u32 {
    let mut n = x.clone();
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }

    count
}

#[inline]
pub fn second_part() -> usize {
    const START_SEQUENCE_AMOUNT: usize = 14;
    let mut buffer = [0; START_SEQUENCE_AMOUNT];

    for (i, x) in include_str!("input.txt")
        .chars()
        .map(|c| c as u32 - 96)
        .enumerate() {

        buffer[i % START_SEQUENCE_AMOUNT] = x;
        if i >= START_SEQUENCE_AMOUNT {
            let bin = buffer
                .iter()
                .fold(0, |acc, x| { acc | 1u32.shl(*x) });

            if count_set_bits(bin) == START_SEQUENCE_AMOUNT as u32 {
                return i+1;
            }
        }
    }

    panic!()
}