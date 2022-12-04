use crate::day_02::Hand::{Paper, Rock, Scissor};
use crate::day_02::Outcome::{Draw, Lose, Win};

enum Hand {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

#[inline]
pub fn first_part() -> u32 {
    include_str!("input.txt")
        .chars()
        .filter(|c| {
            match c {
                'A'..='C' => { true }
                'X'..='Z' => { true }
                _ => { false }
            }
        })
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .fold(0, |acc, x| {
            let op_hand = match x[0] {
                'A' => { Rock }
                'B' => { Paper }
                'C' => { Scissor }
                _ => {panic!("")}
            };
            let my_hand = match x[1] {
                'X' => { Rock }
                'Y'=> { Paper }
                'Z' => { Scissor }
                _ => {panic!("")}
            };

            acc + match (op_hand, my_hand) {
                (Rock, Rock) => {Draw as u32 + Rock as u32}
                (Rock, Paper) => {Win as u32 + Paper as u32}
                (Rock, Scissor) => {Lose as u32 + Scissor as u32}
                (Paper, Rock) => {Lose as u32 + Rock as u32}
                (Paper, Paper) => {Draw as u32 + Paper as u32}
                (Paper, Scissor) => {Win as u32 + Scissor as u32}
                (Scissor, Rock) => {Win as u32 + Rock as u32}
                (Scissor, Paper) => {Lose as u32 + Paper as u32}
                (Scissor, Scissor) => {Draw as u32 + Scissor as u32}
            }
        })
}

#[inline]
pub fn second_part() -> u32 {
    include_str!("input.txt")
        .chars()
        .filter(|c| {
            match c {
                'A'..='C' => { true }
                'X'..='Z' => { true }
                _ => { false }
            }
        })
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .fold(0, |acc, x| {
            let op_hand = match x[0] {
                'A' => { Rock }
                'B' => { Paper }
                'C' => { Scissor }
                _ => unreachable!()
            };

            let my_hand = match x[1] {
                'X' => { Lose }
                'Y'=> { Draw }
                'Z' => { Win }
                _ => unreachable!()
            };

            acc + match (op_hand, my_hand) {
                (Rock, Lose) => { Lose as u32 + Scissor as u32 }
                (Rock, Draw) => { Draw as u32 + Rock as u32 }
                (Rock, Win) => { Win as u32 + Paper as u32 }
                (Paper, Lose) => { Lose as u32 + Rock as u32 }
                (Paper, Draw) => { Draw as u32 + Paper as u32 }
                (Paper, Win) => { Win as u32 + Scissor as u32 }
                (Scissor, Lose) => { Lose as u32 + Paper as u32 }
                (Scissor, Draw) => { Draw as u32 + Scissor as u32 }
                (Scissor, Win) => { Win as u32 + Rock as u32 }
            }
        })
}