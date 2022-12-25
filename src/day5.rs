use std::fs::read_to_string;
use itertools::Itertools;
use regex::Regex;

fn parse_entry(chars: &[char]) -> Option<char> {
    return if chars[1] != ' ' {
        Some(chars[1])
    } else {
        None
    };
}

fn parse_line(line: &str) -> Vec<Option<char>> {
    line.chars()
        .collect_vec()
        .chunks(4)
        .map(parse_entry)
        .collect_vec()
}

fn parse_stacks(content: &str) -> Vec<Vec<Option<char>>> {
    let lines = content.split("\n")
        .collect_vec();
    let (_, stacks) = lines
        .split_last()
        .unwrap();
    stacks.iter()
        .map(|line| parse_line(line))
        .collect_vec()
}

fn build_stacks(stacks_values: &Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let stacks_count = stacks_values.iter().map(|s| s.len()).max().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stacks_count];
    for row in stacks_values {
        for i in 0..row.len() {
            match row[i] {
                None => {}
                Some(val) => stacks[i].insert(0, val)
            }
        }
    }
    stacks
}

fn parse_command(line: &str) -> (usize, usize, usize) {
    let pattern = Regex::new(r"\d+").unwrap();
    pattern.find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn parse_commands(commands: &str) -> Vec<(usize, usize, usize)> {
    commands.split("\n")
        .map(|line| parse_command(line))
        .collect_vec()
}

fn part1(original_stacks: &Vec<Vec<char>>, commands: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = original_stacks.clone();
    for &(count, src, dst) in commands {
        for _ in 0..count {
            let val = stacks[src - 1].pop().unwrap();
            stacks[dst - 1].push(val);
        }
    }
    stacks.iter()
        .map(|s| s.last().unwrap())
        .collect()
}

fn part2(original_stacks: &Vec<Vec<char>>, commands: &Vec<(usize, usize, usize)>) -> String {
    let mut stacks = original_stacks.clone();
    for &(count, src, dst) in commands {
        let mut popped = vec![];
        for _ in 0..count {
            let val = stacks[src - 1].pop().unwrap();
            popped.insert(0, val);
        }
        stacks[dst - 1].extend_from_slice(&mut popped);
    }
    stacks.iter()
        .map(|s| s.last().unwrap())
        .collect()
}

pub(crate) fn solve() {
    let content = read_to_string("5.txt").unwrap();
    let (stacks_section, commands_section) = content.split_once("\n\n").unwrap();
    let stacks_values = parse_stacks(stacks_section);
    let stacks = build_stacks(&stacks_values);
    let commands = parse_commands(commands_section);
    println!("{}", part1(&stacks, &commands));
    println!("{}", part2(&stacks, &commands));
}