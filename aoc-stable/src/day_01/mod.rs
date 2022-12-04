use std::str::FromStr;
use rayon::prelude::ParallelSliceMut;

#[inline]
pub fn first_part() -> u32 {
    let mut amount = 0;
    let mut highest = 0;

    for x in include_str!("input.txt").lines() {
        match u32::from_str(x) {
            Ok(calories) => { // safe because acc always contains a value
                amount += calories;
            }
            Err(_) => {
                if amount > highest {
                    highest = amount;
                }
                amount = 0;
            }
        }
    }

   highest
}

#[inline]
pub fn second_part() -> u32 {
    let mut acc = vec![0];

    for x in include_str!("input.txt").lines() {
        match u32::from_str(x) {
            Ok(calories) => unsafe { // safe because acc always contains a value
                *acc.last_mut().unwrap_unchecked() += calories;
            }
            Err(_) => {
                acc.push(0);
            }
        }
    }

    acc.sort_unstable();

    acc.iter().rev().take(3).sum()
}