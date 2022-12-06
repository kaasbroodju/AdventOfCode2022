use std::str::FromStr;

#[inline]
pub fn first_part() -> String {
    const AMOUNT_OF_BUCKETS: usize = 9;
    const AMOUNT_OF_START_LINES: usize = 8;

    let mut lines = include_str!("input.txt")
        .lines();

    let mut stack = lines
        .by_ref()
        .take(AMOUNT_OF_START_LINES)
        .fold(vec![Vec::with_capacity(AMOUNT_OF_BUCKETS * AMOUNT_OF_BUCKETS); AMOUNT_OF_BUCKETS], |mut acc, line| {
            line
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(pos, x)| {
                    acc[pos].insert(0, x);
                });
            acc
        });

    lines
        .skip(2)
        .map(|x| {
            let splitted = x.split_whitespace().collect::<Vec<_>>();
            (
                usize::from_str(splitted[1]).unwrap(),
                usize::from_str(splitted[3]).unwrap() - 1,
                usize::from_str(splitted[5]).unwrap() - 1
            )
        })
        .fold(stack, |mut acc, (amount, from, to)| {
            let size = acc.get(from).unwrap().len();

            acc
                .get_mut(from)
                .unwrap()
                .drain(size - amount..)
                .rev()
                .collect::<Vec<_>>()
                .iter()
                .for_each(|x| {
                    acc.get_mut(to).unwrap().push(*x);
                });

            acc
        })
        .iter()
        .fold(String::with_capacity(AMOUNT_OF_BUCKETS), |mut acc, row| {
            acc.push(*row.iter().last().unwrap());
            acc
        })
}

#[inline]
pub fn second_part() -> String {
    const AMOUNT_OF_BUCKETS: usize = 9;
    const AMOUNT_OF_START_LINES: usize = 8;

    let mut lines = include_str!("input.txt")
        .lines();

    let mut stack = lines
        .by_ref()
        .take(AMOUNT_OF_START_LINES)
        .fold(vec![Vec::with_capacity(AMOUNT_OF_BUCKETS * AMOUNT_OF_BUCKETS); AMOUNT_OF_BUCKETS], |mut acc, line| {
            line
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .for_each(|(pos, x)| {
                    acc[pos].insert(0, x);
                });
            acc
        });

    lines
        .skip(2)
        .map(|x| {
            let splitted = x.split_whitespace().collect::<Vec<_>>();
            (
                usize::from_str(splitted[1]).unwrap(),
                usize::from_str(splitted[3]).unwrap() - 1,
                usize::from_str(splitted[5]).unwrap() - 1
            )
        })
        .fold(stack, |mut acc, (amount, from, to)| {
            let size = acc.get(from).unwrap().len();

            acc
                .get_mut(from)
                .unwrap()
                .drain(size - amount..)
                .collect::<Vec<_>>()
                .iter()
                .for_each(|x| {
                    acc.get_mut(to).unwrap().push(*x);
                });

            acc
        })
        .iter()
        .fold(String::with_capacity(AMOUNT_OF_BUCKETS), |mut acc, row| {
            acc.push(*row.iter().last().unwrap());
            acc
        })
}