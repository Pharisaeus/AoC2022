use std::fs::read_to_string;
use std::ops::Div;
use itertools::{Itertools, sorted};
use regex::Regex;

enum Op {
    Mul,
    Add,
    Square,
}

struct Monkey {
    items: Vec<i64>,
    divisor: i64,
    test_true: usize,
    test_false: usize,
    op: Op,
    operand: i64,
}

fn compute(item: &i64, op: &Op, operand: i64) -> i64 {
    return match op {
        Op::Mul => item * operand,
        Op::Add => item + operand,
        Op::Square => item * item
    };
}

impl Monkey {
    fn handle_items(&self, reducer: impl Fn(i64) -> i64) -> Vec<(usize, i64)> {
        let mut res = vec![];
        for item in &self.items {
            let new_item = compute(&item, &self.op, self.operand);
            let reduced = reducer(new_item);
            if (reduced % self.divisor) == 0 {
                res.push((self.test_true, reduced));
            } else {
                res.push((self.test_false, reduced));
            }
        }
        res
    }
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    activity: Vec<i64>,
}

impl Monkeys {
    fn play_round(&mut self, reducer: &impl Fn(i64) -> i64) {
        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys.get(i).unwrap();
            let moved_items = monkey.handle_items(reducer);
            self.activity[i] = self.activity[i] + monkey.items.len() as i64;
            self.monkeys.get_mut(i).unwrap().items = vec![];
            for (target_monkey, value) in moved_items {
                self.monkeys.get_mut(target_monkey).unwrap().items.push(value);
            }
        }
    }

    fn activity_score(&self) -> i64 {
        let sorted = sorted(self.activity.iter()).rev().collect_vec();
        sorted[0] * sorted[1]
    }
}

fn parse_numbers(line: &str) -> Vec<i64> {
    let pattern = Regex::new(r"\d+").unwrap();
    pattern.find_iter(line)
        .map(|m| m.as_str().parse().unwrap())
        .collect_vec()
}

fn get_number(line: &str) -> i64 {
    parse_numbers(line).get(0).unwrap().clone()
}

fn parse_monkey(data: &str) -> Monkey {
    let x: (&str, &str, &str, &str, &str) = data.split("\n").skip(1).collect_tuple().unwrap();
    let items = parse_numbers(x.0);
    let operands = parse_numbers(x.1);
    let op;
    if x.1.contains("+") {
        op = Op::Add;
    } else if (x.1.contains("*")) & (operands.len() > 0) {
        op = Op::Mul;
    } else {
        op = Op::Square;
    }
    let operand = operands.get(0).unwrap_or(&0).clone();
    let divisor = get_number(x.2);
    let test_true = get_number(x.3) as usize;
    let test_false = get_number(x.4) as usize;
    Monkey {
        items,
        divisor,
        test_true,
        test_false,
        op,
        operand,
    }
}

fn parse_input(content: &String) -> Monkeys {
    let monkeys = content.split("\n\n")
        .map(|m| parse_monkey(m))
        .collect_vec();
    let count = monkeys.len();
    Monkeys { monkeys, activity: vec![0; count] }
}


fn play(mut monkeys: Monkeys, rounds: i64, reducer: &impl Fn(i64) -> i64) -> i64 {
    for _ in 0..rounds {
        monkeys.play_round(reducer)
    }
    monkeys.activity_score()
}

fn part1(content: &String) -> i64 {
    let monkeys = parse_input(&content);
    play(monkeys, 20, &|x| x.div(3))
}

fn part2(content: &String) -> i64 {
    let monkeys = parse_input(&content);
    let d: i64 = monkeys.monkeys.iter().map(|x| x.divisor).product();
    play(monkeys, 10000, &|x| x % d)
}

pub(crate) fn solve() {
    let content = read_to_string("11.txt").unwrap();
    println!("{}", part1(&content));
    println!("{}", part2(&content));
}