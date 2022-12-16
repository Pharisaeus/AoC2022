use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use itertools::Itertools;
use regex::Regex;

fn floyd_warshall(graph: &HashMap<String, HashSet<String>>) -> HashMap<(&String, &String), i32> {
    let nodes = graph.keys().collect_vec();
    let mut distances = HashMap::new();
    for (k, neighbours) in graph {
        for &n in &nodes {
            let w;
            if k == n {
                w = 0;
            } else if neighbours.contains(n) {
                w = 1;
            } else {
                w = 999999;
            }
            distances.insert((k, n), w);
            distances.insert((n, k), w);
        }
    }
    for &k in &nodes {
        for &i in &nodes {
            for &j in &nodes {
                let existing = distances.get(&(i, j)).unwrap();
                let potential = distances.get(&(i, k)).unwrap() + distances.get(&(k, j)).unwrap();
                distances.insert((i, j), min(*existing, potential));
            }
        }
    }
    distances
}

fn parse_line(line: &str) -> (String, i32, HashSet<String>) {
    let pattern = Regex::new(r"Valve (.*) has flow rate=(\d+); tunnel.? lead.? to valve.? (.*)").unwrap();
    let c = pattern.captures(line).unwrap();
    let neighbours: HashSet<String> = c.get(3).unwrap().as_str().split(", ")
        .map(|x| x.to_string())
        .collect();
    (c.get(1).unwrap().as_str().to_string(), c.get(2).unwrap().as_str().parse().unwrap(), neighbours)
}

fn parse_graph(data: &str) -> (HashMap<String, HashSet<String>>, HashMap<String, i32>) {
    let mut graph = HashMap::new();
    let mut flow = HashMap::new();
    for line in data.split("\n") {
        let (name, rate, neighbours) = parse_line(line);
        graph.insert(name.clone(), neighbours);
        flow.insert(name.clone(), rate);
    }
    (graph, flow)
}

fn find_path(flows: &HashMap<String, i32>, distances: &HashMap<(&String, &String), i32>, steps_left: i32, visited: &HashSet<&String>, current: &String, score: i32, rate: i32) -> i32 {
    return if steps_left <= 0 {
        score
    } else {
        let potential_targets = flows.keys()
            .into_iter()
            .filter(|&v| v != current && !visited.contains(v) && *flows.get(v).unwrap() > 0 && *distances.get(&(current, v)).unwrap() < steps_left)
            .collect_vec();
        let mut best = score + rate * steps_left;
        for t in potential_targets {
            let d = distances.get(&(current, t)).unwrap() + 1;
            let r = flows.get(t).unwrap();
            let new_visited: HashSet<&String> = visited.union(&HashSet::from([current]))
                .map(|x| *x)
                .collect();
            let s = find_path(flows, distances, steps_left - d, &new_visited, t, score + rate * d, rate + r);
            if s > best {
                best = s;
            }
        }
        best
    };
}

fn find_dual_path(flows: &HashMap<String, i32>, distances: &HashMap<(&String, &String), i32>,
                  steps_left_me: i32, steps_left_el: i32, visited: &HashSet<&String>,
                  current_me: &String, current_el: &String,
                  score_me: i32, score_el: i32, rate_me: i32, rate_el: i32) -> i32 {
    let potential_targets_me = flows.keys()
        .into_iter()
        .filter(|&v| v != current_me && v != current_el && !visited.contains(v) && *flows.get(v).unwrap() > 0 && *distances.get(&(current_me, v)).unwrap() < steps_left_me)
        .collect_vec();
    let potential_targets_el = flows.keys()
        .into_iter()
        .filter(|&v| v != current_el && v != current_me && !visited.contains(v) && *flows.get(v).unwrap() > 0 && *distances.get(&(current_el, v)).unwrap() < steps_left_el)
        .collect_vec();
    let new_visited: HashSet<&String> = visited.union(&HashSet::from([current_el, current_me]))
        .map(|x| *x)
        .collect();
    return if potential_targets_me.len() == 0 && potential_targets_el.len() == 0 {
        score_me + score_el + steps_left_el * rate_el + steps_left_me * rate_me
    } else if potential_targets_me.len() == 0 && potential_targets_el.len() > 0 {
        find_path(flows, distances, steps_left_el, &new_visited, current_el, score_me + steps_left_me * rate_me + score_el, rate_el)
    } else if potential_targets_el.len() == 0 && potential_targets_me.len() > 0 {
        find_path(flows, distances, steps_left_me, &new_visited, current_me, score_el + steps_left_el * rate_el + score_me, rate_me)
    } else {
        let mut best = score_me + score_el + rate_me * steps_left_me + rate_el * steps_left_el;
        for my_target in potential_targets_me {
            let d_me = distances.get(&(current_me, my_target)).unwrap() + 1;
            let r_me = flows.get(my_target).unwrap();
            for &el_target in &potential_targets_el {
                if my_target != el_target {
                    let d_el = distances.get(&(current_el, el_target)).unwrap() + 1;
                    let r_el = flows.get(el_target).unwrap();
                    let s = find_dual_path(flows, distances, steps_left_me - d_me, steps_left_el - d_el,
                                           &new_visited, my_target,
                                           el_target, score_me + rate_me * d_me, score_el + rate_el * d_el, rate_me + r_me, rate_el + r_el);
                    if s > best {
                        best = s;
                    }
                }
            }
        }
        best
    };
}

fn part2(flows: &HashMap<String, i32>, distances: &HashMap<(&String, &String), i32>) -> i32 {
    find_dual_path(flows, distances, 26, 26, &HashSet::new(), &"AA".to_string(), &"AA".to_string(), 0, 0, 0, 0)
}

fn part1(flows: &HashMap<String, i32>, distances: &HashMap<(&String, &String), i32>) -> i32 {
    find_path(flows, distances, 30, &HashSet::new(), &"AA".to_string(), 0, 0)
}

pub(crate) fn solve() {
    let content = read_to_string("16.txt").unwrap();
    let (graph, flows) = parse_graph(&content);
    let distances = floyd_warshall(&graph);
    println!("{}", part1(&flows, &distances));
    println!("{}", part2(&flows, &distances));
}