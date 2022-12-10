use std::borrow::Borrow;
use std::collections::HashSet;
use std::iter::{Enumerate, Map};
use std::ops::{BitOr, BitOrAssign, Shl};
use std::str::{Chars, FromStr};

#[inline]
pub fn first_part() -> usize {
    const START_SEQUENCE_AMOUNT: usize = 4;
    let mut buffer = [0; START_SEQUENCE_AMOUNT];

    include_str!("input.txt")
        .chars()
        .map(|c| c as u32 - 96)
        .enumerate()
        .find(|(i, x)| {
            buffer[i % START_SEQUENCE_AMOUNT] = *x;
            if *i < START_SEQUENCE_AMOUNT {return false}
            let mut acc = 0;
            for x in buffer {
                let pos_char = 1u32.shl(x);
                if acc & pos_char > 0 {
                    return false;
                } else {
                    acc.bitor_assign(pos_char);
                }

            }
            return true;
        }).unwrap().0 + 1
}

#[inline]
pub fn second_part() -> usize {
    const START_SEQUENCE_AMOUNT: usize = 14;
    let mut buffer = [0; START_SEQUENCE_AMOUNT];

    include_str!("input.txt")
        .chars()
        .map(|c| c as u32 - 96)
        .enumerate()
        .find(|(i, x)| {
            buffer[i % START_SEQUENCE_AMOUNT] = *x;
            if *i < START_SEQUENCE_AMOUNT {return false}
            let mut acc = 0;
            for x in buffer {
                let pos_char = 1u32.shl(x);
                if acc & pos_char > 0 {
                    return false;
                } else {
                    acc.bitor_assign(pos_char);
                }

            }
            return true;
        }).unwrap().0 + 1
}