use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::usize;

#[inline]
pub fn first_part() -> i32 {
    let mut i = 0;
    let mut register = 1;
    let mut acc = 0;

    include_str!("input.txt")
        .lines()
        .for_each(|line| {
            match &line[0..4] {
                "noop" => {
                    i += 1;
                    if (i - 20) % 40 == 0 {
                        acc += register * i;
                    }
                }
                "addx" => {
                    i+=1;
                    if (i - 20) % 40 == 0 {
                        acc += register * i;
                    }
                    i+=1;
                    if (i - 20) % 40 == 0 {
                        acc += register * i;
                    }
                    register += i32::from_str(&line[5..]).unwrap();
                }
                _ => panic!()
            }
        });

    acc
}

#[inline]
pub fn second_part() -> String {
    const SCREEN_WIDTH: i32 = 40;
    let mut acc = String::with_capacity(240);
    let mut i = 0;
    let mut register: i32 = 1;

    include_str!("input.txt")
        .lines()
        .for_each(|line| {
            match &line[0..4] {
                "noop" => {
                    acc.push(if does_overlap(i % SCREEN_WIDTH, register) { '#' } else { '.' } );
                    i += 1;
                }
                "addx" => {
                    acc.push(if does_overlap(i  % SCREEN_WIDTH, register) { '#' } else { '.' } );
                    i+=1;
                    acc.push(if does_overlap(i  % SCREEN_WIDTH, register) { '#' } else { '.' } );
                    i+=1;
                    register += i32::from_str(&line[5..]).unwrap();
                }
                _ => { panic!() }
            }
        });

    // for (i, c) in acc.chars().enumerate() {
    //     if i % 40 == 0 { println!()}
    //     print!("{c}");
    // }
    // println!();

    acc
}

#[inline]
fn does_overlap(pos: i32, i: i32) -> bool {
    pos == i-1 || pos == i || pos == i + 1
}