use std::collections::HashSet;
use std::fs::read_to_string;
use itertools::Itertools;

fn parse_input(content: &str) -> Vec<(i32, i32, i32)> {
    content.split("\n")
        .map(|line| line.split(",")
            .map(|x| x.parse().unwrap())
            .collect_tuple().unwrap())
        .collect_vec()
}

fn neighbouring_cubes(cube: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let (x, y, z) = cube;
    let mut res = vec![];
    for shift in [-1, 1] {
        let pos = vec![(x + shift, *y, *z), (*x, y + shift, *z), (*x, *y, z + shift)];
        for c in pos {
            let (nx, ny, nz) = c;
            let min = -1;
            let max = 22;
            if (nx >= min) & (nx <= max) & (ny >= min) & (ny <= max) & (nz >= min) & (nz <= max) {
                res.insert(0, c);
            }
        }
    }
    res
}

fn uncovered_sides(cube: &(i32, i32, i32), taken_slots: &HashSet<&(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    neighbouring_cubes(cube)
        .iter()
        .filter(|&c| !taken_slots.contains(c))
        .map(|x| *x)
        .collect_vec()
}

fn bfs(src: (i32, i32, i32), taken_slots: &HashSet<&(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let mut visited = HashSet::new();
    let mut to_process = vec![src];
    while to_process.len() > 0 {
        let m = to_process.pop().unwrap();
        for neighbour in neighbouring_cubes(&m) {
            if !visited.contains(&neighbour) & !taken_slots.contains(&neighbour) {
                visited.insert(neighbour);
                to_process.insert(0, neighbour);
            }
        }
    }
    visited
}

fn external_cubes(cooling_cubes: &Vec<(i32, i32, i32)>, taken_slots: &HashSet<&(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let reachable = bfs((0, 0, 0), taken_slots);
    cooling_cubes
        .iter()
        .filter(|&c| reachable.contains(c))
        .map(|c| *c)
        .collect()
}

fn part2(cubes: &Vec<(i32, i32, i32)>) -> usize {
    let taken_slots = HashSet::from_iter(cubes);
    let cooling_cubes = cubes.iter()
        .flat_map(|&cube| uncovered_sides(&cube, &taken_slots))
        .collect_vec();
    let external = external_cubes(&cooling_cubes, &taken_slots);
    cooling_cubes.iter()
        .filter(|&c| external.contains(c))
        .count()
}

fn part1(cubes: &Vec<(i32, i32, i32)>) -> usize {
    let taken_slots = HashSet::from_iter(cubes);
    cubes.iter()
        .flat_map(|&cube| uncovered_sides(&cube, &taken_slots))
        .count()
}

pub(crate) fn solve() {
    let content = read_to_string("18.txt").unwrap();
    let cubes = parse_input(&content);
    println!("{}", part1(&cubes));
    println!("{}", part2(&cubes));
}