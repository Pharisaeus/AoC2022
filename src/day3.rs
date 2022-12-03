use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn item_score(item: &char) -> i32 {
    match item {
        'a'..='z' => (*item as i32) - ('a' as i32) + 1,
        'A'..='Z' => (*item as i32) - ('A' as i32) + 27,
        _ => panic!()
    }
}

fn intersect(first: Vec<char>, second: Vec<char>) -> Vec<char> {
    let first: HashSet<char> = HashSet::from_iter(first.to_vec());
    let second: HashSet<char> = HashSet::from_iter(second.to_vec());
    first.intersection(&second).map(|x| *x).collect_vec()
}

fn find_common(rucksack: &Vec<char>) -> i32 {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);
    let common = intersect(left.to_vec(), right.to_vec());
    common.first()
        .map(item_score)
        .unwrap()
}

fn part1(rucksacks: &Vec<Vec<char>>) -> i32 {
    rucksacks.iter()
        .map(find_common)
        .sum()
}

fn find_badge(triplet: Vec<Vec<char>>) -> i32 {
    triplet.into_iter()
        .reduce(|x, y| intersect(x, y))
        .unwrap()
        .first()
        .map(item_score)
        .unwrap()
}

fn part2(rucksacks: &Vec<Vec<char>>) -> i32 {
    rucksacks.chunks(3)
        .map(|triplet| triplet.to_vec())
        .map(find_badge)
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("3.txt").unwrap();
    let rucksacks = contents.split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    println!("{}", part1(&rucksacks));
    println!("{}", part2(&rucksacks));
}