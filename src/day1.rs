use std::fs;
use itertools::{Itertools, sorted};

fn elf_calories(elf: &Vec<i32>) -> i32 {
    elf.iter().sum()
}

fn elfs_calories(elfs: &Vec<Vec<i32>>) -> Vec<i32> {
    elfs.iter()
        .map(elf_calories)
        .collect()
}

fn part1(elfs: &Vec<Vec<i32>>) -> i32 {
    elfs_calories(elfs)
        .iter()
        .max()
        .copied()
        .unwrap()
}

fn part2(elfs: &Vec<Vec<i32>>) -> i32 {
    sorted(elfs_calories(elfs))
        .rev()
        .take(3)
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("1.txt").unwrap();
    let elfs: Vec<Vec<i32>> = contents.split("\n\n")
        .map(|elf| elf.split("\n").map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();
    println!("{}", part1(&elfs));
    println!("{}", part2(&elfs));
}
