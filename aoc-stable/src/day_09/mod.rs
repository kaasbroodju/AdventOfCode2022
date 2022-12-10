use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::usize;
use crate::day_09::Direction::{Down, Left, Right, Up};

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[inline]
pub fn first_part() -> usize {
    let mut tail = (0,0);
    let mut head = (0,0);

    let mut overlaps = HashSet::new();

    include_str!("input.txt")
        .lines()
        .fold(vec![], |mut acc, line| {
            acc.extend(vec![
                match &line[0..1] {
                    "U" => Up,
                    "R" => Right,
                    "D" => Down,
                    "L" => Left,
                    _ => panic!()
                }; line[2..].parse::<usize>().unwrap()
            ]);
            acc
        })
        .iter()
        .for_each(|direction| {
            match direction {
                Left => { head.0 -= 1; }
                Right => { head.0 += 1; }
                Up => { head.1 += 1; }
                Down => { head.1 -= 1; }
            }

            match direction {
                Left => { if tail.0 - head.0 > 1 { tail.0 = head.0 + 1; tail.1 = head.1; } }
                Right => { if head.0 - tail.0 > 1 { tail.0 = head.0 - 1; tail.1 = head.1; } }
                Up => { if head.1 - tail.1 > 1 { tail.1 = head.1 - 1; tail.0 = head.0; } }
                Down => { if tail.1 - head.1 > 1 { tail.1 = head.1 + 1; tail.0 = head.0; } }
            }

            overlaps.insert(tail);
        });

    overlaps.len()
}

#[inline]
pub fn second_part() -> usize {

    const AMOUNT_OF_KNOTS: usize = 10;
    let mut tails: [(i32, i32); AMOUNT_OF_KNOTS] = [(0,0); AMOUNT_OF_KNOTS];

    let mut overlaps = HashSet::new();

    include_str!("input.txt")
        .lines()
        .fold(vec![], |mut acc, line| {
            acc.extend(vec![
                match &line[0..1] {
                    "U" => Up,
                    "R" => Right,
                    "D" => Down,
                    "L" => Left,
                    _ => panic!()
                }; line[2..].parse::<usize>().unwrap()
            ]);
            acc
        })
        .iter()
        .for_each(|direction| {
            match direction {
                Left => { tails[0].0 -= 1; }
                Right => { tails[0].0 += 1; }
                Up => { tails[0].1 += 1; }
                Down => { tails[0].1 -= 1; }
            }

            (1..AMOUNT_OF_KNOTS)
                .for_each(|i|  {
                    let temp_head = tails[i-1];
                    let temp_tail = &mut tails[i];

                    let delta_knots = (temp_head.0 - temp_tail.0, temp_head.1 - temp_tail.1);

                    if delta_knots.0.abs() == 2 && delta_knots.1 == 0 {
                        // horizontal
                        temp_tail.0 += delta_knots.0 / delta_knots.0.abs();
                    } else if delta_knots.1.abs() == 2 && delta_knots.0 == 0 {
                        // vertical
                        temp_tail.1 += delta_knots.1 / delta_knots.1.abs();
                    } else if delta_knots.0.abs() + delta_knots.1.abs() == 3 {
                        // diagonal (with horizontal or vertical)
                        if delta_knots.0 != 0 { temp_tail.0 += delta_knots.0 / delta_knots.0.abs(); }
                        if delta_knots.1 != 0 { temp_tail.1 += delta_knots.1 / delta_knots.1.abs(); }

                    } else if delta_knots.0.abs() == 2 && delta_knots.1.abs() == 2 {
                        // diagonal
                        temp_tail.0 += delta_knots.0 / delta_knots.0.abs();
                        temp_tail.1 += delta_knots.1 / delta_knots.1.abs();
                    }
                });
            overlaps.insert(tails[AMOUNT_OF_KNOTS-1]);
        });

    overlaps.len()
}