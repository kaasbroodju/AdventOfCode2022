#[inline]
pub fn first_part() -> usize {
    const OFFSET: u32 = '0' as u32;
    let trees = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars().map(|c| u32::from(c) - OFFSET).collect::<Vec<u32>>()
        }).collect::<Vec<Vec<u32>>>();

    let mut acc = 0;
    for col in 0..trees.len() {
        for row in 0..trees[col].len() {
            if (0..row).into_iter().all(|i| trees[col][row] > trees[col][i])
                || (row+1..trees.len()).into_iter().all(|i| trees[col][row] > trees[col][i])
                || (0..col).into_iter().all(|i| trees[col][row] > trees[i][row])
                || (col+1..trees[col].len()).into_iter().all(|i| trees[col][row] > trees[i][row]) {
                acc += 1;
            }
        }
    }

    acc
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