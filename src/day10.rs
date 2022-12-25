use std::fs::read_to_string;
use itertools::Itertools;

fn simulate(operations: &Vec<&str>) -> Vec<i32> {
    let mut register_history = vec![1];
    for &op in operations {
        let last = register_history.last().unwrap().clone();
        match op {
            "noop" => register_history.push(last),
            add => {
                let v: i32 = add.split_once(" ").unwrap().1.parse().unwrap();
                register_history.push(last);
                register_history.push(last + v);
            }
        }
    }
    return register_history;
}

fn part2(operations: &Vec<&str>) {
    let register_history = simulate(operations);
    let mut screen = vec![];
    for cycle in 0..register_history.len() {
        let sprite_middle = register_history.get(cycle).unwrap();
        if (cycle as i32 % 40 - sprite_middle).abs() <= 1 {
            screen.push("#");
        } else {
            screen.push(" ");
        }
    }
    for pixel in 0..screen.len() {
        if pixel % 40 == 0 {
            println!()
        }
        print!("{}", screen.get(pixel).unwrap())
    }
    println!()
}

fn part1(operations: &Vec<&str>) -> i32 {
    let register_history = simulate(operations);
    (20..register_history.len()).step_by(40)
        .map(|cycle| (cycle as i32) * register_history.get((cycle - 1) as usize).unwrap())
        .sum()
}

pub(crate) fn solve() {
    let content = read_to_string("10.txt").unwrap();
    let operations = content.split("\n")
        .collect_vec();
    println!("{}", part1(&operations));
    part2(&operations)
}