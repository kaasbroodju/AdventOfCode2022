use std::collections::{HashMap, HashSet};
use std::ops::{Div, DivAssign, Shl};
use std::str::FromStr;
use std::usize;
use crate::day_11::Operation::{ValueAdd, ValueMul, SelfAdd, SelfMul};

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_dividor: u64,
    throw_to_monkey_x_when_true: usize,
    throw_to_monkey_x_when_false: usize,
}

enum Operation {
    SelfAdd,
    ValueAdd(u64),
    SelfMul,
    ValueMul(u64)
}

#[inline]
pub fn first_part() -> u64 {
    const ROUNDS: usize = 20;
    let mut monkeys = vec![];

    let mut lines = include_str!("input.txt").lines();

    loop {
        lines.next(); // monkey ignore
        let items = lines.next().unwrap()[18..].split(", ").map(|n| u64::from_str(n).unwrap()).collect::<Vec<u64>>();

        let opline = lines.next().unwrap()[19..].split_whitespace().collect::<Vec<&str>>();
        let operation = match (opline[0], opline[1], opline[2]) {
            ("old", "+", "old") => {  SelfAdd }
            ("old", "+", v) =>  { ValueAdd(u64::from_str(v).unwrap()) }
            ("old", "*", "old") =>  { SelfMul }
            ("old", "*", v) =>  { ValueMul(u64::from_str(v).unwrap()) }
            (a, b, c) => { panic!("{a}:{b}:{c}") }
        };

        let dividor = u64::from_str(&lines.next().unwrap()[21..]).unwrap();
        let when_true = usize::from_str(&lines.next().unwrap()[29..]).unwrap();
        let when_false = usize::from_str(&lines.next().unwrap()[30..]).unwrap();
        monkeys.push(Monkey {
            items,
            operation,
            test_dividor: dividor,
            throw_to_monkey_x_when_true: when_true,
            throw_to_monkey_x_when_false: when_false,
        });
        if lines.next().is_none() {break;}
    }

    let mut monkey_activity = [0;8];
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.drain(..).collect::<Vec<_>>() {
                monkey_activity[i] += 1;

                let x = match monkeys[i].operation {
                    SelfAdd => { item + item }
                    ValueAdd(v) => { item + v }
                    SelfMul => { item * item }
                    ValueMul(v) => { item * v }
                } / 3;

                let new_location = if x % monkeys[i].test_dividor == 0 {
                    monkeys[i].throw_to_monkey_x_when_true
                } else {
                    monkeys[i].throw_to_monkey_x_when_false
                };
                monkeys[new_location].items.push(x);
            }
        }
    }

    monkey_activity.sort();

    monkey_activity.iter().rev().take(2).fold(1, |acc, e| { acc * e })
}

#[inline]
pub fn second_part() -> u64 {
    const ROUNDS: usize = 10_000;

    let mut monkeys = vec![];

    let mut lines = include_str!("input.txt").lines();

    loop {
        lines.next(); // monkey ignore
        let items = lines.next().unwrap()[18..].split(", ").map(|n| u64::from_str(n).unwrap()).collect::<Vec<u64>>();

        let opline = lines.next().unwrap()[19..].split_whitespace().collect::<Vec<&str>>();
        let operation = match (opline[0], opline[1], opline[2]) {
            ("old", "+", "old") => {  SelfAdd }
            ("old", "+", v) =>  { ValueAdd(u64::from_str(v).unwrap()) }
            ("old", "*", "old") =>  { SelfMul }
            ("old", "*", v) =>  { ValueMul(u64::from_str(v).unwrap()) }
            (a, b, c) => { panic!("{a}:{b}:{c}") }
        };

        let dividor = u64::from_str(&lines.next().unwrap()[21..]).unwrap();
        let when_true = usize::from_str(&lines.next().unwrap()[29..]).unwrap();
        let when_false = usize::from_str(&lines.next().unwrap()[30..]).unwrap();
        monkeys.push(Monkey {
            items,
            operation,
            test_dividor: dividor,
            throw_to_monkey_x_when_true: when_true,
            throw_to_monkey_x_when_false: when_false,
        });
        if lines.next().is_none() {break;};
    }

    let overflow_preventer = monkeys.iter().map(|m| m.test_dividor).fold(1, |acc, e| acc * e);

    let mut monkey_activity = [0u64;8];
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.drain(..).collect::<Vec<_>>() {
                monkey_activity[i] += 1;

                let x = match monkeys[i].operation {
                    SelfAdd => { item + item }
                    ValueAdd(v) => { item + v }
                    SelfMul => { item * item }
                    ValueMul(v) => { item * v }
                } % overflow_preventer;

                let new_location = if x % monkeys[i].test_dividor == 0 {
                    monkeys[i].throw_to_monkey_x_when_true
                } else {
                    monkeys[i].throw_to_monkey_x_when_false
                };
                monkeys[new_location].items.push(x);
            }
        }
    }

    monkey_activity.sort();

    monkey_activity.iter().rev().take(2).fold(1, |acc, e| { acc * e })
}