use std::cmp::max;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::panic::resume_unwind;
use itertools::Itertools;
use regex::Regex;

struct Position {
    row: i64,
    col: i64,
}

impl Position {
    fn distance(&self, other: &Position) -> i64 {
        (self.row - other.row).abs() + (self.col - other.col).abs()
    }
}

struct Sensor {
    sensor: Position,
    beacon: Position,
    max_distance: i64,
}

impl Sensor {
    pub(crate) fn impossible_columns(&self, row: i64) -> Option<(i64, i64)> {
        let steps_left_to_do = self.max_distance - (self.sensor.row - row).abs();
        return if steps_left_to_do > 0 {
            let left = self.sensor.col - steps_left_to_do;
            let right = self.sensor.col + steps_left_to_do;
            Some((left, right))
        } else {
            None
        };
    }
}

impl Sensor {
    fn new(line: &str) -> Sensor {
        let pattern = Regex::new(r"-?\d+").unwrap();
        let (s_col, s_row, b_col, b_row) = pattern.find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect_tuple()
            .unwrap();
        let sensor = Position { row: s_row, col: s_col };
        let beacon = Position { row: b_row, col: b_col };
        let max_distance = sensor.distance(&beacon);
        Sensor {
            sensor,
            beacon,
            max_distance,
        }
    }

    fn edge(&self) -> Vec<(i64, i64)> {
        let row_up = ((self.sensor.row - self.max_distance - 1)..self.sensor.row + 1).collect_vec();
        let row_down = (self.sensor.row..(self.sensor.row + self.max_distance + 1)).collect_vec();
        let col_left = ((self.sensor.col - self.max_distance - 1)..self.sensor.col + 1).collect_vec();
        let col_right = (self.sensor.col..(self.sensor.col + self.max_distance + 1)).collect_vec();

        let left_up = row_up.iter().rev().zip(col_left.iter()).collect_vec();
        let left_down = row_down.iter().zip(col_left.iter()).collect_vec();
        let right_up = row_up.iter().zip(col_right.iter()).collect_vec();
        let right_down = row_down.iter().rev().zip(col_left.iter()).collect_vec();

        let mut result = vec![];
        result.extend(left_up);
        result.extend(left_down);
        result.extend(right_up);
        result.extend(right_down);
        result.iter().map(|&(row, col)| (*row, *col)).collect_vec()
    }

    fn is_not_possible(&self, position: &Position) -> bool {
        position.distance(&self.sensor) <= self.max_distance
    }
}

fn parse_sensors(data: &str) -> Vec<Sensor> {
    data.split("\n")
        .map(Sensor::new)
        .collect_vec()
}

fn overlaps(r1: &(i64, i64), r2: &(i64, i64)) -> bool {
    r1.1 >= r2.0
}

fn check_position(position: &Position, sensors: &Vec<Sensor>) -> bool {
    for sensor in sensors {
        if sensor.is_not_possible(position) {
            return false;
        }
    }
    return true;
}

fn part2(sensors: &Vec<Sensor>) -> i64 {
    for sensor in sensors {
        for (row, col) in sensor.edge() {
            if row >= 0 && row <= 4000000 && col >= 0 && col <= 4000000 {
                let p = Position { row, col };
                if check_position(&p, sensors) {
                    return col * 4000000 + row;
                }
            }
        }
    }
    0
}

fn part1(sensors: &Vec<Sensor>) -> i64 {
    let sorted_ranges = sensors.iter()
        .map(|s| s.impossible_columns(2000000))
        .filter(|r| r.is_some())
        .map(|x| x.unwrap())
        .sorted_by(|x, y| x.0.cmp(&y.0))
        .collect_vec();
    let mut values = 0;
    let mut current = *sorted_ranges.get(0).unwrap();
    for range in sorted_ranges.iter().skip(1) {
        if overlaps(&current, range) {
            current = (current.0, max(current.1, range.1))
        } else {
            values += (current.1 - current.0);
            current = *range;
        }
    }
    values += (current.1 - current.0);
    values
}

pub(crate) fn solve() {
    let content = read_to_string("15.txt").unwrap();
    let sensors = parse_sensors(&content);
    println!("{}", part1(&sensors));
    println!("{}", part2(&sensors));
}