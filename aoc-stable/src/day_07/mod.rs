use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::iter::{Enumerate, Map};
use std::ops::{BitOr, BitOrAssign, Shl};
use std::str::{Chars, FromStr};
use std::time::Instant;

struct Directory {
    directories: Vec<String>, // acts as pointers
    computed_size: usize, // sum of file sizes
}

impl Directory {
    fn new() -> Directory {
        Directory {directories: vec![], computed_size: 0}
    }
}

#[inline]
pub fn first_part() -> usize {
    let mut pwd = vec![];

    let pwd_as_string = |v: &Vec<&str>| {
        v.iter().fold(String::new(), |mut acc, e| {
            acc.push_str(e);
            acc
        })
    };

    let mut directories: HashMap<String, Directory> = HashMap::from(
        [(String::from("/"), Directory::new())]
    );

    for splitted in include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
    {
        if splitted[0] == "$" {
            if splitted[1] == "cd" {
                if splitted[2] == ".." {
                    pwd.pop();
                } else {
                    pwd.push(splitted[2]);
                }
            }
        } else if splitted[0] == "dir" {
            directories.insert(pwd_as_string(&pwd) + splitted[1], Directory::new());
            directories.get_mut(&*pwd_as_string(&pwd)).unwrap().directories.push(pwd_as_string(&pwd) + splitted[1]);
        } else {
            let x = splitted[0].parse::<usize>().unwrap();
            directories.get_mut(&*pwd_as_string(&pwd)).unwrap().computed_size += x;
        }
    }

    directories
        .keys()
        .filter_map(|dir_name| {
            let x = calculate_amount_of_dir(&directories, dir_name);
            if x < 100000 { Some(x) } else { None }
        }).sum()
}

fn calculate_amount_of_dir(directories: &HashMap<String, Directory>, dir_name: &String) -> usize {
    let computed = directories
        .get(dir_name)
        .unwrap()
        .computed_size;

    let inside = directories
        .get(dir_name)
        .unwrap()
        .directories
        .iter()
        .map(|dir| calculate_amount_of_dir(directories, dir)).sum::<usize>();

    inside + computed
}

#[inline]
pub fn second_part() -> usize {
    let mut pwd = vec![];

    let pwd_as_string = |v: &Vec<&str>| {
        v.iter().fold(String::new(), |mut acc, e| {
            acc.push_str(e);
            acc
        })
    };

    let mut directories: HashMap<String, Directory> = HashMap::from(
        [(String::from("/"), Directory::new())]
    );

    for line in include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
    {
        if line[0] == "$" {
            if line[1] == "cd" {
                if line[2] == ".." {
                    pwd.pop();
                } else {
                    pwd.push(line[2]);
                }
            }
        } else if line[0] == "dir" {
            directories.insert(pwd_as_string(&pwd) + line[1], Directory::new());
            directories.get_mut(&*pwd_as_string(&pwd)).unwrap().directories.push(pwd_as_string(&pwd) + line[1]);
        } else {
            let x = line[0].parse::<usize>().unwrap();
            directories.get_mut(&*pwd_as_string(&pwd)).unwrap().computed_size += x;
        }
    }

    const TARGET_THRESHOLD: usize = 30000000;
    const AVAILABLE_SPACE: usize = 70000000;
    let threshold = AVAILABLE_SPACE - calculate_amount_of_dir(&directories, &String::from("/"));
    directories
        .keys()
        .filter_map(|dir_name| {
            let x = calculate_amount_of_dir(&directories, dir_name);

            if threshold + x > TARGET_THRESHOLD { Some(x) } else { None }
        })
        .min()
        .unwrap()
}