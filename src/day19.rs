use std::fs::read_to_string;
use itertools::Itertools;
use regex::Regex;

struct Blueprint {
    ore_ore: i8,
    clay_ore: i8,
    obsidian_ore: i8,
    obsidian_clay: i8,
    geode_ore: i8,
    geode_obsidian: i8,
}

fn ceil(x: i8, y: i8) -> i8 {
    let z = x / y;
    let v = x % y;
    return if z < 0 {
        -1
    } else if v != 0 {
        z + 1
    } else {
        z
    };
}

impl Blueprint {
    fn new(line: &str) -> Blueprint {
        let pattern = Regex::new(r"\d+").unwrap();
        let (_, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) = pattern.find_iter(line)
            .map(|m| m.as_str().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Blueprint {
            ore_ore,
            clay_ore,
            obsidian_ore,
            obsidian_clay,
            geode_ore,
            geode_obsidian,
        }
    }
    fn produce_ore_robot(&self, state: &State) -> Option<(State, i8)> {
        return if (state.ore_robots > 3) | (state.ore > 4) {
            None
        } else if state.ore >= self.ore_ore {
            let mut s = State::make_from(&state.progress_one_round());
            s.ore -= self.ore_ore;
            s.ore_robots += 1;
            Some((s, 1))
        } else {
            let mut next_state = state.clone();
            let missing_ore = self.ore_ore - state.ore;
            let rounds = ceil(missing_ore, state.ore_robots) + 1;
            for _ in 0..rounds {
                next_state = next_state.progress_one_round();
            }
            let mut s = State::make_from(&next_state);
            s.ore -= self.ore_ore;
            s.ore_robots += 1;
            Some((s, rounds))
        };
    }

    fn produce_clay_robot(&self, state: &State) -> Option<(State, i8)> {
        return if (state.ore_robots > 6) | (state.clay > 25) {
            None
        } else if state.ore >= self.clay_ore {
            let mut s = State::make_from(&state.progress_one_round());
            s.ore -= self.clay_ore;
            s.clay_robots += 1;
            Some((s, 1))
        } else {
            let mut next_state = state.clone();
            let missing_ore = self.clay_ore - state.ore;
            let rounds = ceil(missing_ore, state.ore_robots) + 1;
            for _ in 0..rounds {
                next_state = next_state.progress_one_round();
            }
            let mut s = State::make_from(&next_state);
            s.ore -= self.clay_ore;
            s.clay_robots += 1;
            Some((s, rounds))
        };
    }

    fn produce_obsidian_robot(&self, state: &State) -> Option<(State, i8)> {
        return if (state.obsidian_robots > 6) | (state.obsidian > 25) {
            None
        } else if (state.ore >= self.obsidian_ore) & (state.clay >= self.obsidian_clay) {
            let mut s = State::make_from(&state.progress_one_round());
            s.ore -= self.obsidian_ore;
            s.clay -= self.obsidian_clay;
            s.obsidian_robots += 1;
            Some((s, 1))
        } else if state.clay_robots > 0 {
            let mut next_state = state.clone();
            let missing_ore = self.obsidian_ore - state.ore;
            let missing_clay = self.obsidian_clay - state.clay;
            let rounds = ceil(missing_ore, state.ore_robots).max(ceil(missing_clay, state.clay_robots)) + 1;
            for _ in 0..rounds {
                next_state = next_state.progress_one_round();
            }
            let mut s = State::make_from(&next_state);
            s.ore -= self.obsidian_ore;
            s.clay -= self.obsidian_clay;
            s.obsidian_robots += 1;
            Some((s, rounds))
        } else {
            None
        };
    }

    fn produce_geode_robot(&self, state: &State) -> Option<(State, i8)> {
        if (state.ore >= self.geode_ore) & (state.obsidian >= self.geode_obsidian) {
            let mut s = State::make_from(&state.progress_one_round());
            s.ore -= self.geode_ore;
            s.obsidian -= self.geode_obsidian;
            s.geodes_robots += 1;
            Some((s, 1))
        } else if state.obsidian_robots > 0 {
            let mut next_state = state.clone();
            let missing_ore = self.geode_ore - state.ore;
            let missing_obsidian = self.geode_obsidian - state.obsidian;
            let rounds = ceil(missing_ore, state.ore_robots).max(ceil(missing_obsidian, state.obsidian_robots)) + 1;
            for _ in 0..rounds {
                next_state = next_state.progress_one_round();
            }
            let mut s = State::make_from(&next_state);
            s.ore -= self.geode_ore;
            s.obsidian -= self.geode_obsidian;
            s.geodes_robots += 1;
            Some((s, rounds))
        } else {
            None
        }
    }

    fn generate_possible_robots(&self, state: &State, minutes: &i8) -> Vec<(State, i8)> {
        vec![self.produce_geode_robot(state), self.produce_obsidian_robot(state), self.produce_clay_robot(state), self.produce_ore_robot(state)]
            .iter()
            .filter(|&x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .filter(|(_state, m)| m <= minutes)
            .map(|x| x.clone())
            .collect_vec()
    }
}

#[derive(Clone)]
struct State {
    ore: i8,
    ore_robots: i8,
    clay: i8,
    clay_robots: i8,
    obsidian: i8,
    obsidian_robots: i8,
    geodes: i8,
    geodes_robots: i8,
}

impl State {
    fn progress_one_round(&self) -> State {
        State {
            ore: self.ore + self.ore_robots,
            ore_robots: self.ore_robots,
            clay: self.clay + self.clay_robots,
            clay_robots: self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            obsidian_robots: self.obsidian_robots,
            geodes: self.geodes + self.geodes_robots,
            geodes_robots: self.geodes_robots,
        }
    }
    fn make_from(state: &State) -> State {
        state.clone()
    }

    fn default() -> State {
        State {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geodes_robots: 0,
        }
    }
}

fn find(blueprint: &Blueprint, minutes: &i8, state: &State) -> i8 {
    return if *minutes <= 0 {
        state.geodes
    } else {
        let next_states = blueprint.generate_possible_robots(state, minutes);
        if next_states.len() > 0 {
            next_states
                .iter()
                .map(|(s, m)| find(&blueprint, &(minutes - m), s))
                .max()
                .unwrap()
        } else {
            state.geodes + state.geodes_robots * minutes
        }
    };
}

fn parse_blueprints(content: &str) -> Vec<Blueprint> {
    content
        .split("\n")
        .map(|line| Blueprint::new(line))
        .collect_vec()
}

fn part1(blueprints: &Vec<Blueprint>) -> i32 {
    let mut res: i32 = 0;
    for i in 0..blueprints.len() {
        let result = find(blueprints.get(i).unwrap(), &24, &State::default());
        res += (i + 1) as i32 * result as i32;
    }
    res
}


fn part2(blueprints: &Vec<Blueprint>) -> i32 {
    let mut res: i32 = 1;
    for i in 0..3 {
        let result = find(blueprints.get(i).unwrap(), &32, &State::default());
        res *= result as i32
    }
    res
}

pub(crate) fn solve() {
    let content = read_to_string("19.txt").unwrap();
    let blueprints = parse_blueprints(&content);
    println!("{}", part1(&blueprints));
    println!("{}", part2(&blueprints));
}