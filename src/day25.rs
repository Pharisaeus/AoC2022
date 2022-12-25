use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::iter::Sum;
use std::ops::Add;
use itertools::Itertools;

struct ElfNumber {
    value: i64,
}

fn digit_value(digit: &char) -> i64 {
    match digit {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!()
    }
}

fn value_digit(v: i64) -> char {
    match v {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => panic!()
    }
}

fn multiplier(position: usize) -> i64 {
    let mut res = 1;
    for _ in 0..position {
        res *= 5;
    }
    res
}

impl ElfNumber {
    fn new(line: &str) -> ElfNumber {
        let digits = line.chars().rev().collect_vec();
        let mut value = 0;
        for position in 0..digits.len() {
            let c = digits.get(position).unwrap();
            value += digit_value(c) * multiplier(position)
        }
        ElfNumber { value }
    }
    fn add(&self, other: &ElfNumber) -> ElfNumber {
        ElfNumber {
            value: self.value + other.value
        }
    }
}

impl ToString for ElfNumber {
    fn to_string(&self) -> String {
        let mut res = vec![];
        let mut value = self.value;
        while value > 0 {
            let x = value + 2;
            let digit = x.rem_euclid(5) - 2;
            value = x.div_euclid(5);
            res.insert(0, value_digit(digit));
        }
        res.iter().join("")
    }
}

fn part1(numbers: &Vec<ElfNumber>) -> String {
    let mut result = ElfNumber { value: 0 };
    for n in numbers {
        result = result.add(n);
    }
    result.to_string()
}

pub(crate) fn solve() {
    let content = read_to_string("25.txt").unwrap();
    let numbers = content.split("\n")
        .map(|line| ElfNumber::new(line))
        .collect_vec();
    println!("{}", part1(&numbers));
}