use std::collections::HashSet;
use std::fs::read_to_string;
use itertools::Itertools;

fn distance((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> i32 {
    let distances = [(ax - bx).abs(), (ay - by).abs()];
    distances.iter()
        .max()
        .unwrap()
        .clone()
}

fn norm(value: i32) -> i32 {
    return if value != 0 {
        value / value.abs()
    } else {
        0
    };
}

fn fix_tail((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    return if distance((hx, hy), (tx, ty)) <= 1 {
        (tx, ty)
    } else {
        let vx = norm(hx - tx);
        let vy = norm(hy - ty);
        (tx + vx, ty + vy)
    };
}

fn move_head((hx, hy): (i32, i32), direction: &str) -> (i32, i32) {
    return match direction {
        "U" => (hx, hy + 1),
        "D" => (hx, hy - 1),
        "L" => (hx - 1, hy),
        "R" => (hx + 1, hy),
        &_ => panic!()
    };
}

fn part2(commands: &Vec<(&str, i32)>) -> i32 {
    let mut knots = (0..10).map(|_| (0, 0)).collect_vec();
    let mut visited = HashSet::new();
    for (direction, steps) in commands {
        for _ in 0..*steps {
            knots[0] = move_head(knots[0], direction);
            for i in 0..knots.len() - 1 {
                knots[i + 1] = fix_tail(knots[i], knots[i + 1])
            }
            visited.insert(knots[knots.len() - 1]);
        }
    }
    visited.len() as i32
}

fn part1(commands: &Vec<(&str, i32)>) -> i32 {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();
    for (direction, steps) in commands {
        for _ in 0..*steps {
            head = move_head(head, direction);
            tail = fix_tail(head, tail);
            visited.insert(tail);
        }
    }
    visited.len() as i32
}

pub(crate) fn solve() {
    let content = read_to_string("9.txt").unwrap();
    let commands = content.split("\n")
        .map(|line| line.split_once(" ").unwrap())
        .map(|(direction, steps)| (direction, steps.parse().unwrap()))
        .collect_vec();
    println!("{}", part1(&commands));
    println!("{}", part2(&commands));
}