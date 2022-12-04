use std::ops::{Shl};

#[inline]
fn calc_amount(a: char) -> u32 {
    let result = a as u32;
    if result > 96 {
        result - 96
    } else {
        result - 38
    }

}

#[inline]
fn translate_slice(slice: &str) -> u64 {
    let mut amount = 0u64;

    for x in slice.chars() {
        amount |= 1u64.shl(calc_amount(x));
    }

    amount
}

#[inline]
fn pos_of_first_one(n: u64) -> u32 {
    let mut position = 0u32;
    let mut m = 1u64;

    while (n & m) == 0 {
        m <<= 1;
        position+=1;
    }

    return position;
}


#[inline]
pub fn first_part() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let splitpoint = line.len() / 2;

            let a = translate_slice(&line[..splitpoint]);
            let b = translate_slice(&line[splitpoint..]);

            pos_of_first_one(a & b)
        })
        .sum()
}

#[inline]
pub fn second_part() -> u32 {
    include_str!("input.txt")
        .lines()
        .map(|slice| { translate_slice(slice) })
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|x| pos_of_first_one(x[0] & x[1] & x[2]))
        .sum()
}