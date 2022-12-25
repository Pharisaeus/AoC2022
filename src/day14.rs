use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::read_to_string;
use itertools::Itertools;

struct Bounds {
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
}

struct Board {
    board: HashMap<(i32, i32), bool>,
    bounds: Bounds,
    added_sand: i32,
}

impl Board {
    fn new(data: &Vec<Vec<(i32, i32)>>) -> Board {
        let mut board = HashMap::new();
        for line in data {
            for i in 0..line.len() - 1 {
                let start = line.get(i).unwrap();
                let end = line.get(i + 1).unwrap();
                let min_col = min(start.0, end.0);
                let max_col = max(start.0, end.0);
                let min_row = min(start.1, end.1);
                let max_row = max(start.1, end.1);
                for row in min_row..max_row + 1 {
                    for col in min_col..max_col + 1 {
                        board.insert((row, col), true);
                    }
                }
            }
        }
        let bounds = Board::compute_bounds(&board);
        Board { board, bounds, added_sand: 0 }
    }

    fn compute_bounds(board: &HashMap<(i32, i32), bool>) -> Bounds {
        let rows = board.keys().map(|k| k.0).collect_vec();
        let cols = board.keys().map(|k| k.1).collect_vec();
        let min_row = *rows.iter().min().unwrap();
        let max_row = *rows.iter().max().unwrap();
        let min_col = *cols.iter().min().unwrap();
        let max_col = *cols.iter().max().unwrap();
        return Bounds { min_row, max_row, min_col, max_col };
    }

    fn pos_out_of_bounds(&self, (row, col): (i32, i32)) -> bool {
        row > self.bounds.max_row || col < self.bounds.min_col || col > self.bounds.max_col
    }

    fn on_the_floor(&self, row: i32) -> bool {
        return row == self.bounds.max_row + 2;
    }

    fn simple_move(&self, (row, col): (i32, i32)) -> Option<(i32, i32)> {
        return if !self.board.contains_key(&(row + 1, col)) {
            Some((row + 1, col))
        } else if !self.board.contains_key(&(row + 1, col - 1)) {
            Some((row + 1, col - 1))
        } else if !self.board.contains_key(&(row + 1, col + 1)) {
            Some((row + 1, col + 1))
        } else {
            Some((row, col))
        };
    }

    fn next_move(&self, (row, col): (i32, i32)) -> Option<(i32, i32)> {
        return if self.pos_out_of_bounds((row, col)) {
            None
        } else {
            self.simple_move((row, col))
        };
    }

    fn next_move_with_floor(&self, (row, col): (i32, i32)) -> Option<(i32, i32)> {
        return if self.on_the_floor(row + 1) {
            Some((row, col))
        } else {
            let next_move = self.simple_move((row, col));
            return match next_move {
                None => panic!(),
                Some((0, 500)) => None,
                any => any
            };
        };
    }

    fn mark(&mut self, pos: (i32, i32)) {
        self.board.insert(pos, true);
        self.added_sand += 1;
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let bounds = &self.bounds;
        let mut res = String::new();
        for row in bounds.min_row - 5..bounds.max_row + 3 {
            for col in bounds.min_col - 10..bounds.max_col + 10 {
                if self.board.contains_key(&(row, col)) {
                    res += "#";
                } else {
                    res += ".";
                }
            }
            res += "\n";
        }
        res
    }
}

fn parse_coords(coords: &str) -> (i32, i32) {
    let (row, col) = coords.split_once(",").unwrap();
    (row.parse().unwrap(), col.parse().unwrap())
}

fn parse_line(line: &str) -> Vec<(i32, i32)> {
    line.split(" -> ")
        .map(parse_coords)
        .collect_vec()
}

fn parse_data(data: &str) -> Vec<Vec<(i32, i32)>> {
    data.split("\n")
        .map(parse_line)
        .collect_vec()
}


fn play(mut board: Board, next_move: fn(&Board, (i32, i32)) -> Option<(i32, i32)>) -> i32 {
    loop {
        let mut pos = (0, 500);
        loop {
            let new_pos = next_move(&board, pos);
            match new_pos {
                None => {
                    return board.added_sand;
                }
                Some(new_position) => {
                    if new_position == pos {
                        board.mark(pos);
                        break;
                    } else {
                        pos = new_position
                    }
                }
            }
        }
    }
}

fn part1(board: Board) -> i32 {
    play(board, Board::next_move)
}

fn part2(board: Board) -> i32 {
    play(board, Board::next_move_with_floor) + 1
}

pub(crate) fn solve() {
    let content = read_to_string("14.txt").unwrap();
    let data = parse_data(&content);
    println!("{}", part1(Board::new(&data)));
    println!("{}", part2(Board::new(&data)));
}