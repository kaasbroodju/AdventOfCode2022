use std::borrow::BorrowMut;
use rayon::prelude::*;

#[inline]
pub fn first_part() -> usize {
    const OFFSET: u32 = '0' as u32;
    let trees = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars().map(|c| (u32::from(c) - OFFSET) as u8)
        })
        .flatten()
        .collect::<Vec<u8>>();

    let length = trees.len();
    let size = (length as f64).sqrt() as usize;

    (0..length)
        .into_par_iter()
        .zip(&trees)
        .filter(|(i, x)| {
            (*i-(*i % size)..*i).all(|index| **x > trees[index])
            || ((*i % size)..*i).step_by(size).all(|index| **x > trees[index])
            || (*i+1..*i-(*i % size)+size).all(|index| **x > trees[index])
            || (*i+size..length).step_by(size).all(|index| **x > trees[index])
        })
        .count()
}

#[inline]
pub fn second_part() -> usize {
    const OFFSET: u32 = '0' as u32;
    let trees = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars().map(|c| u32::from(c) - OFFSET).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();

    let mut acc = vec![];
    for col in 1..trees.len()-1 {
        for row in 1..trees[col].len()-1 {
            let left = (0..row)
                .into_iter()
                .rev()
                .find_map(|i| {
                    if trees[col][row] <= trees[col][i] {
                        Some(row - i)
                    } else {
                        None
                    }
                })
                .unwrap_or(row);

            let right = (row + 1..trees.len())
                .into_iter()
                .find_map(|i| {
                    if trees[col][row] <= trees[col][i]  {
                        Some(i - row)
                    } else {
                        None
                    }
                })
                .unwrap_or(trees.len() - 1 - row);

            let top = (0..col)
                .into_iter()
                .rev()
                .find_map(|i| {
                    if trees[col][row] <= trees[i][row] {
                        Some(col - i)
                    } else {
                        None
                    }
                })
                .unwrap_or(col);

            let bottom = (col +  1..trees[col].len())
                .into_iter()
                .find_map(|i| {
                    if trees[col][row] <= trees[i][row] {
                        Some(i - col)
                    } else {
                        None
                    }
                })
                .unwrap_or(trees[row].len() - 1  - col);

            acc.push(left * right * top * bottom);
        }
    }
    *acc.iter().max().unwrap()
}