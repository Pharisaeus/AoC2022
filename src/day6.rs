use std::fs::read_to_string;
use std::ops::Index;
use itertools::Itertools;

fn all_unique(window: &[char]) -> bool {
    window.iter()
        .unique()
        .count() == window.len()
}

fn find_marker(content: &str, block_size: usize) -> usize {
    let letters = content.chars()
        .collect_vec();
    let code: String = letters
        .windows(block_size)
        .find_or_first(|window| all_unique(window))
        .unwrap()
        .iter()
        .collect();
    content.find(&code).unwrap() + block_size
}

fn part1(content: &str) -> usize {
    find_marker(content, 4)
}

fn part2(content: &str) -> usize {
    find_marker(content, 14)
}

pub(crate) fn solve() {
    let content = read_to_string("6.txt").unwrap();
    println!("{}", part1(&content));
    println!("{}", part2(&content));
}

