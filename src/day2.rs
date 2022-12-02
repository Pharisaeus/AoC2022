use std::fs;
use itertools::Itertools;

fn get_move_points(choice: &str) -> i32 {
    match choice {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!()
    }
}

fn get_round_points(first: &str, second: &str) -> i32 {
    return if get_winning_move(first) == second {
        6
    } else if first == second {
        3
    } else {
        0
    };
}

fn map_move(choice: &str) -> &str {
    match choice {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => panic!()
    }
}

fn get_game_score(rounds: Vec<(&str, String)>) -> i32 {
    rounds.iter()
        .map(|(opponent, me)| get_move_points(me) + get_round_points(opponent, me))
        .sum()
}

fn get_winning_move(choice: &str) -> &str {
    match choice {
        "A" => "B",
        "B" => "C",
        "C" => "A",
        _ => panic!()
    }
}

fn get_losing_move(choice: &str) -> &str {
    get_winning_move(get_winning_move(choice))
}


fn get_move_to_play(choice: &str, result_type: &str) -> String {
    match result_type {
        "X" => get_losing_move(choice),
        "Y" => choice,
        "Z" => get_winning_move(choice),
        &_ => panic!()
    }.to_string()
}

fn part1(rounds: &Vec<(&str, &str)>) -> i32 {
    let played_rounds = rounds.iter()
        .map(|&(opponent, me)| (opponent, map_move(me).to_string()))
        .collect_vec();
    get_game_score(played_rounds)
}

fn part2(rounds: &Vec<(&str, &str)>) -> i32 {
    let played_rounds = rounds.iter()
        .map(|&(opponent, result_type)| (opponent, get_move_to_play(opponent, result_type)))
        .collect_vec();
    get_game_score(played_rounds)
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("2.txt").unwrap();
    let rounds = contents.split("\n")
        .map(|line| line.split_once(" ").unwrap())
        .collect_vec();
    println!("{}", part1(&rounds));
    println!("{}", part2(&rounds));
}