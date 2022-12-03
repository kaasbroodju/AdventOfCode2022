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
            let (rug_a, rug_b) = line.split_at(line.len() / 2);

            let a = translate_slice(rug_a);
            let b = translate_slice(rug_b);

            pos_of_first_one(a & b)

        })
        .sum::<u32>()
}

#[inline]
pub fn second_part() -> u32 {
    let x =include_str!("input.txt")
        .lines()
        .collect::<Vec<&str>>();

    let mut amount = 0u32;
    for i in (0..x.len()).step_by(3) {
        let (rug_a, rug_b, rug_c) = (x[i], x[i+1], x[i+2]);

        let a = translate_slice(rug_a);
        let b = translate_slice(rug_b);
        let c = translate_slice(rug_c);

        amount += pos_of_first_one(a & b & c);
    }
    amount
}