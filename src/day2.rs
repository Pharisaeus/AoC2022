use std::fs;
use itertools::Itertools;
use crate::day2::RPS::{Paper, Rock, Scissors};

#[derive(Copy, Clone, PartialEq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RPS {
    pub fn new(choice: &str) -> Self {
        match choice {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!()
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Result {
    pub fn new(choice: &str) -> Self {
        match choice {
            "X" => Result::Lose,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!()
        }
    }
}

fn get_round_result(opponent: &RPS, me: &RPS) -> Result {
    return if get_winning_move(opponent) == *me {
        Result::Win
    } else if opponent == me {
        Result::Draw
    } else {
        Result::Lose
    };
}


fn get_round_score(opponent: RPS, me: RPS) -> i32 {
    (me as i32) + (get_round_result(&opponent, &me) as i32)
}

fn get_winning_move(choice: &RPS) -> RPS {
    match choice {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}

fn get_losing_move(choice: &RPS) -> RPS {
    get_winning_move(&get_winning_move(choice))
}


fn get_move_to_play(opponent: &RPS, result_type: &Result) -> RPS {
    match result_type {
        Result::Win => get_winning_move(opponent),
        Result::Draw => *opponent,
        Result::Lose => get_losing_move(opponent),
    }
}

fn part1(rounds: &Vec<(&str, &str)>) -> i32 {
    rounds.iter()
        .map(|&(opponent, me)| (RPS::new(opponent), RPS::new(me)))
        .map(|(opponent, me)| get_round_score(opponent, me))
        .sum()
}

fn part2(rounds: &Vec<(&str, &str)>) -> i32 {
    rounds.iter()
        .map(|&(opponent, result_type)| (RPS::new(opponent), Result::new(result_type)))
        .map(|(opponent, result)| (opponent, get_move_to_play(&opponent, &result)))
        .map(|(opponent, me)| get_round_score(opponent, me))
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("2.txt").unwrap();
    let rounds = contents.split("\n")
        .map(|line| line.split_once(" ").unwrap())
        .collect_vec();
    println!("{}", part1(&rounds));
    println!("{}", part2(&rounds));
}