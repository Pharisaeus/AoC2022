use std::collections::VecDeque;
use std::fs::read_to_string;
use itertools::Itertools;

fn encrypt(data: &Vec<i64>, rounds: i8) -> Vec<i64> {
    let with_position = (0..data.len())
        .map(|index| (index, *data.get(index).unwrap()))
        .collect_vec();
    let mut res = VecDeque::from(with_position.clone());
    for _ in 0..rounds {
        for entry in &with_position {
            let &(_, value) = entry;
            let current_index = res.iter().position(|v| v == entry).unwrap();
            res.remove(current_index);
            let shifts = (value % res.len() as i64);
            if shifts > 0 {
                res.rotate_left(shifts.abs() as usize);
            } else {
                res.rotate_right(shifts.abs() as usize);
            }
            res.insert(current_index, *entry);
        }
    }
    res.iter()
        .map(|(_, value)| *value)
        .collect_vec()
}

fn get_result(encrypted: &Vec<i64>) -> i64 {
    let start = encrypted.iter().position(|&x| x == 0).unwrap();
    [1000, 2000, 3000].iter()
        .map(|x| encrypted.get((start + x) % encrypted.len()).unwrap())
        .sum()
}

fn part1(data: &Vec<i64>) -> i64 {
    get_result(&encrypt(data, 1))
}

fn part2(data: &Vec<i64>) -> i64 {
    let multiplied_data = data.iter()
        .map(|v| v * 811589153)
        .collect_vec();
    get_result(&encrypt(&multiplied_data, 10))
}

pub(crate) fn solve() {
    let content = read_to_string("20.txt").unwrap();
    let numbers = content
        .split("\n")
        .map(|line| line.parse().unwrap())
        .collect_vec();
    println!("{}", part1(&numbers));
    println!("{}", part2(&numbers));
}