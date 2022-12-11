use std::collections::{HashMap, HashSet};
use std::ops::{Div, DivAssign};
use std::str::FromStr;
use std::usize;

#[derive(Clone, Debug)]
struct Monkey<'a> {
    items: Vec<u64>,
    operation: (&'a str, &'a str, &'a str),
    test_dividor: u64,
    throw_to_monkey_x_when_true: usize,
    throw_to_monkey_x_when_false: usize,
}

#[inline]
pub fn first_part() -> u64 {
    let mut monkeys = vec![];

    let mut lines = include_str!("input.txt").lines();

    loop {
        lines.next(); // monkey ignore
        let items = lines.next().unwrap()[18..].split(", ").map(|n| u64::from_str(n).unwrap()).collect::<Vec<u64>>();
        let idk = lines.next().unwrap()[19..].split_whitespace().collect::<Vec<&str>>();


        let operation = (idk[0], idk[1], idk[2]);

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


    let mut monkey_activity = [0;9];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let current_monkey = monkeys.get(i).unwrap().clone();
            let i_when_true = current_monkey.throw_to_monkey_x_when_true;
            let i_when_false = current_monkey.throw_to_monkey_x_when_false;
            for item in current_monkey.items {
                monkey_activity[i] += 1;
                let (a, op, b) = current_monkey.operation;
                let x = performe_fn(a, op, b, item);
                let x = (x as f64).div(3.0).floor() as u64;

                if x % current_monkey.test_dividor == 0 {
                    monkeys.get_mut(i_when_true).unwrap().items.push(x);
                } else {
                    monkeys.get_mut(i_when_false).unwrap().items.push(x);
                }
            }

            monkeys.get_mut(i).unwrap().items.clear();

        }
    }

    monkey_activity.sort();

    monkey_activity.iter().rev().take(2).fold(1, |acc, e| { acc * e })
}

fn performe_fn(a: &str, op: &str, c: &str, x: u64) -> u64 {
    match (a, op, c) {
        ("old", "+", "old") => {  x + x }
        ("old", "+", v) =>  { x + u64::from_str(v).unwrap() }
        ("old", "*", "old") =>  { x * x }
        ("old", "*", v) =>  { x * u64::from_str(v).unwrap() }
        (a, b, c) => { panic!("{a}:{b}:{c}") }
    }
}

#[inline]
pub fn second_part() -> u64 {
    let mut monkeys = vec![];

    let mut lines = include_str!("input.txt").lines();

    loop {
        lines.next(); // monkey ignore
        let items = lines.next().unwrap()[18..].split(", ").map(|n| u64::from_str(n).unwrap()).collect::<Vec<u64>>();
        let idk = lines.next().unwrap()[19..].split_whitespace().collect::<Vec<&str>>();

        let operation = (idk[0], idk[1], idk[2]);

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
    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            let current_monkey = monkeys.get_mut(i).unwrap().clone();
            let i_when_true = current_monkey.throw_to_monkey_x_when_true;
            let i_when_false = current_monkey.throw_to_monkey_x_when_false;
            for item in current_monkey.items.iter() {
                monkey_activity[i] += 1;
                let (a, op, b) = current_monkey.operation;
                let x = performe_fn(a, op, b, *item) % overflow_preventer;

                if x % current_monkey.test_dividor == 0 {
                    monkeys.get_mut(i_when_true).unwrap().items.push(x);
                } else {
                    monkeys.get_mut(i_when_false).unwrap().items.push(x);
                }
            }

            monkeys.get_mut(i).unwrap().items.clear();

        }
    }

    monkey_activity.sort();

    monkey_activity.iter().rev().take(2).fold(1, |acc, e| { acc * e })
}