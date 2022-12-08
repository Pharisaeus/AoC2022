use std::fs::read_to_string;
use itertools::Itertools;

fn left(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<i32> {
    (0..col)
        .map(|j| grid.get(row).unwrap().get(j).unwrap().clone())
        .rev()
        .collect_vec()
}

fn right(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<i32> {
    (col + 1..grid.len())
        .map(|j| grid.get(row).unwrap().get(j).unwrap().clone())
        .collect_vec()
}

fn up(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<i32> {
    (0..row)
        .map(|i| grid.get(i).unwrap().get(col).unwrap().clone())
        .rev()
        .collect_vec()
}

fn down(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<i32> {
    (row + 1..grid.get(row).unwrap().len())
        .map(|i| grid.get(i).unwrap().get(col).unwrap().clone())
        .collect_vec()
}

fn get_highest_tree_in_each_direction(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> Vec<i32> {
    let l = left(grid, row, col);
    let r = right(grid, row, col);
    let u = up(grid, row, col);
    let d = down(grid, row, col);
    [l, r, u, d].map(|x| x.iter().max().unwrap_or(&-1).clone())
        .to_vec()
}

fn is_visible(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let tree_height = grid.get(row).unwrap().get(col).unwrap().clone();
    let m = get_highest_tree_in_each_direction(grid, row, col)
        .iter()
        .min()
        .unwrap()
        .clone();
    m < tree_height
}

fn part1(grid: &Vec<Vec<i32>>) -> i32 {
    (0..grid.len())
        .flat_map(|row| (0..grid.get(row).unwrap().len())
            .filter(move |col| is_visible(grid, row, *col)))
        .count() as i32
}

fn count_smaller(height: &i32, others: &Vec<i32>) -> i32 {
    let pos = others.iter()
        .find_position(|x| *x >= height)
        .map(|(pos, _)| pos + 1)
        .unwrap_or(others.len());
    pos as i32
}

fn calculate_score(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> i32 {
    let tree_height = grid.get(row).unwrap().get(col).unwrap();
    let l = left(grid, row, col);
    let r = right(grid, row, col);
    let u = up(grid, row, col);
    let d = down(grid, row, col);
    [l, r, u, d]
        .iter()
        .map(|x| count_smaller(tree_height, x))
        .fold(1, |a, b| a * b)
}

fn part2(grid: &Vec<Vec<i32>>) -> i32 {
    (0..grid.len())
        .flat_map(|row| (0..grid.get(row).unwrap().len())
            .map(move |col| calculate_score(grid, row, col)))
        .max()
        .unwrap()
}

pub(crate) fn solve() {
    let content = read_to_string("8.txt").unwrap();
    let grid: Vec<Vec<i32>> = content.split("\n")
        .map(|line| line.chars().map(|c| c.to_string().parse().unwrap()).collect_vec())
        .collect_vec();
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}