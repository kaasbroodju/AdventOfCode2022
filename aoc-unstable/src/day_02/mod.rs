use std::ops::{Add, BitAnd, BitOr, Not, Rem, Sub};
use std::simd::{i8x64, Mask, Simd, SimdPartialEq, SimdUint, u8x64};

#[inline]
// https://github.com/arnokamphuis/AdventOfCode/blob/branch-2022/2022/src/day02_22.rs
pub fn first_part() -> u32 {
    let (a, b) = include_str!("input.txt")
        .lines()
        .fold((vec![], vec![]), |mut acc, x| {
            acc.0.push(
                x.chars().nth(0).unwrap() as i8 - 65
            );
            acc.1.push(
                x.chars().nth(2).unwrap() as i8 - 88
            );
            acc
        });


    let (prefix_a, middle_a, suffix_a) = a.as_simd::<64>();
    let (prefix_b, middle_b, suffix_b) = b.as_simd::<64>();

    let simd_zeros = i8x64::splat(0);
    let simd_ones = i8x64::splat(1);
    let simd_twos = i8x64::splat(2);
    let simd_threes = i8x64::splat(3);
    let simd_sixes = i8x64::splat(6);
    let mut acc = 0;

    for (a, b) in middle_a.iter().zip(middle_b) {
        let deltas = a.sub(b);
        let rem_euclid = deltas.rem(simd_threes).add(simd_threes).rem(simd_threes);

        let mask_twos = rem_euclid.simd_eq(simd_twos);
        let mask_zeros = rem_euclid.simd_eq(simd_zeros);

        let result = mask_twos.select(simd_sixes, simd_zeros);
        let score = mask_zeros.select(simd_threes, result);

        let output = simd_ones.add(b).add(score);

        acc += output.as_array().iter().map(|e| *e as u32).sum::<u32>();
    }
    do_fixes(prefix_a, prefix_b) + acc + do_fixes(suffix_a, suffix_b)
}

fn do_fixes(prefix_a: &[i8], prefix_b: &[i8]) -> u32 {
    let diff = | p1, p2 | -> u32 {
        (p1 as i8 - p2 as i8).rem_euclid(3) as u32
    };

    let score = | p1, p2 | -> u32 {
        match diff(p1,p2) {
            2 => 6,
            0 => 3,
            _ => 0,
        }
    };

    prefix_a.iter().zip(prefix_b).fold(0, |s, (p1,p2)| {
        s + 1 + *p2 as u32 + score(*p1,*p2)
    })
}

#[inline]
pub fn second_part() -> u32 {
    0
}