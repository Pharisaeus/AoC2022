use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    row: i32,
    col: i32,
}

struct Graph {
    edges: HashMap<Node, Vec<Node>>,
    start: Node,
    end: Node,
    low: Vec<Node>,
}

impl Graph {
    fn height(symbol: &char) -> i32 {
        let x = match symbol {
            'S' => 'a',
            'E' => 'z',
            a => *a
        };
        x as i32 - 'a' as i32
    }

    fn node_symbol(node: &Node, grid: &Vec<Vec<char>>) -> char {
        grid.get(node.row as usize).unwrap().get(node.col as usize).unwrap().clone()
    }

    fn node_height(node: &Node, grid: &Vec<Vec<char>>) -> i32 {
        Graph::height(&Graph::node_symbol(node, grid))
    }

    fn height_diff(n1: &Node, n2: &Node, grid: &Vec<Vec<char>>) -> i32 {
        let current = Graph::node_height(n1, grid);
        let next = Graph::node_height(n2, grid);
        next - current
    }

    fn valid_index(row: i32, col: i32, grid: &Vec<Vec<char>>) -> bool {
        (row >= 0) && (col >= 0) && (row < grid.len() as i32) && (col < grid.get(row as usize).unwrap().len() as i32)
    }

    fn legal_moves(node: &Node, grid: &Vec<Vec<char>>) -> Vec<Node> {
        let moves: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        moves.iter()
            .filter(|(vr, vc)| Graph::valid_index(node.row + vr, node.col + vc, grid))
            .map(|(vr, vc)| Node { row: node.row + vr, col: node.col + vc })
            .filter(|n2| Graph::height_diff(node, n2, grid) <= 1)
            .collect_vec()
    }

    fn new(content: &String) -> Graph {
        let grid = content.split("\n")
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let mut start = Node { row: 0, col: 0 };
        let mut end = Node { row: 0, col: 0 };
        let mut low = vec![];
        let mut edges = HashMap::new();
        for row in 0..grid.len() {
            for col in 0..grid.get(row).unwrap().len() {
                let symbol = Graph::node_symbol(&Node { row: row as i32, col: col as i32 }, &grid);
                let node = Node { row: row as i32, col: col as i32 };
                if symbol == 'S' {
                    start = node;
                    low.insert(0, node)
                } else if symbol == 'E' {
                    end = node;
                } else if symbol == 'a' {
                    low.insert(0, node)
                }
                edges.insert(node, Graph::legal_moves(&node, &grid));
            }
        }
        Graph {
            edges,
            start,
            end,
            low,
        }
    }
    fn backtrace(&self, src: &Node, prev: HashMap<&Node, &Node>) -> Vec<Node> {
        let mut res = vec![];
        let mut current = &self.end;
        while current != src {
            res.insert(0, *current);
            current = prev.get(current).unwrap();
        }
        res.insert(0, *src);
        res
    }

    fn bfs(&self, src: &Node) -> Option<Vec<Node>> {
        let mut visited = HashSet::new();
        let mut to_process = vec![src];
        let mut backtrack = HashMap::new();
        while to_process.len() > 0 {
            let m = to_process.pop().unwrap();
            for neighbour in self.edges.get(m).unwrap() {
                if !visited.contains(neighbour) {
                    visited.insert(neighbour);
                    to_process.insert(0, neighbour);
                    backtrack.insert(neighbour, m);
                    if neighbour == &self.end {
                        return Some(self.backtrace(src, backtrack));
                    }
                }
            }
        }
        None
    }
}

fn part2(graph: &Graph) -> i32 {
    graph.low.iter()
        .map(|src| graph.bfs(src))
        .filter(|path| path.is_some())
        .map(|path| path.unwrap())
        .map(|path| path.len() as i32)
        .min()
        .unwrap() - 1
}

fn part1(graph: &Graph) -> i32 {
    let path = graph.bfs(&graph.start).unwrap();
    path.len() as i32 - 1
}

pub(crate) fn solve() {
    let content = read_to_string("12.txt").unwrap();
    let graph = Graph::new(&content);
    println!("{}", part1(&graph));
    println!("{}", part2(&graph));
}