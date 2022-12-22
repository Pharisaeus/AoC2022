use std::fs::read_to_string;
use itertools::Itertools;
use regex::Regex;

#[derive(Clone, Copy)]
enum Move {
    Step(i32),
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        }
    }

    fn score(&self) -> i64 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3
        }
    }

    fn left_turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }

    fn right_turn(&self) -> Direction {
        self.left_turn().left_turn().left_turn()
    }
}

struct Board {
    board: Vec<Vec<char>>,
    direction: Direction,
    row: i32,
    col: i32,
}

impl Board {
    fn new(data: &str) -> Board {
        Board {
            board: data.split("\n").map(|line| line.chars().collect_vec()).collect_vec(),
            direction: Direction::Right,
            row: 0,
            col: 50,
        }
    }

    fn reset(&mut self) {
        self.row = 0;
        self.col = 50;
        self.direction = Direction::Right;
    }

    fn score(&self) -> i64 {
        (self.row + 1) as i64 * 1000 + 4 * (self.col + 1) as i64 + self.direction.score()
    }

    fn tile(&self, row: i32, col: i32) -> Option<char> {
        self.board.get(row as usize)
            .map(|row| row.get(col as usize).map(|c| c.clone()))
            .flatten()
            .filter(|&c| c != ' ')
    }

    fn make_move(&mut self, m: Move, wrap: &impl Fn(&Board) -> (i32, i32, Direction)) {
        match m {
            Move::Step(steps) => self.move_forward(steps, wrap),
            Move::Left => self.direction = self.direction.left_turn(),
            Move::Right => self.direction = self.direction.right_turn()
        }
    }

    fn move_forward(&mut self, steps: i32, wrap: &impl Fn(&Board) -> (i32, i32, Direction)) {
        for _ in 0..steps {
            let (new_row, new_col, new_dir) = self.single_step(wrap);
            if self.tile(new_row, new_col).unwrap() == '#' {
                break;
            } else {
                self.row = new_row;
                self.col = new_col;
                self.direction = new_dir;
            }
        }
    }

    fn single_step(&self, wrap: &impl Fn(&Board) -> (i32, i32, Direction)) -> (i32, i32, Direction) {
        let (vr, vc) = self.direction.move_vector();
        let (new_row, new_col) = (self.row + vr, self.col + vc);
        return match self.tile(new_row, new_col) {
            None => wrap(self),
            Some(_) => (new_row, new_col, self.direction)
        };
    }
}

fn parse_board(board: &str) -> Board {
    Board::new(board)
}

fn parse_move(m: &str) -> Move {
    match m {
        "L" => Move::Left,
        "R" => Move::Right,
        number => Move::Step(number.parse().unwrap())
    }
}

fn parse_moves(moves: &str) -> Vec<Move> {
    let pattern = Regex::new(r"[LR]|\d+").unwrap();
    pattern.find_iter(moves)
        .map(|m| parse_move(m.as_str()))
        .collect_vec()
}

fn parse_inputs(content: &str) -> (Board, Vec<Move>) {
    let (board, moves) = content.split_once("\n\n")
        .unwrap();
    (parse_board(board), parse_moves(moves))
}

fn make_moves(board: &mut Board, moves: &Vec<Move>, wrap: &impl Fn(&Board) -> (i32, i32, Direction)) -> i64 {
    board.reset();
    for m in moves {
        board.make_move(*m, wrap);
    }
    board.score()
}

fn wrap_flat(board: &Board) -> (i32, i32, Direction) {
    let opposite_direction = board.direction.left_turn().left_turn();
    let (vr, vc) = opposite_direction.move_vector();
    let (mut r, mut c) = (board.row, board.col);
    loop {
        let (rr, cc) = (r + vr, c + vc);
        if board.tile(rr, cc).is_none() {
            return (r, c, board.direction);
        } else {
            r = rr;
            c = cc;
        }
    }
}

fn wrap_3d(board: &Board) -> (i32, i32, Direction) {
    let size = 50;
    let r = board.row / size;
    let c = board.col / size;
    match (r, c, board.direction) {
        // 1
        (0, 1, Direction::Up) => (100 + board.col, 0, Direction::Right),
        (0, 1, Direction::Left) => (149 - board.row, 0, Direction::Right),
        // 2
        (0, 2, Direction::Up) => (199, board.col - 100, Direction::Up),
        (0, 2, Direction::Right) => (149 - board.row, 99, Direction::Left),
        (0, 2, Direction::Down) => (board.col - 50, 99, Direction::Left),
        // 3
        (1, 1, Direction::Left) => (100, board.row - 50, Direction::Down),
        (1, 1, Direction::Right) => (49, 50 + board.row, Direction::Up),
        // 4
        (2, 0, Direction::Up) => (board.col + 50, 50, Direction::Right),
        (2, 0, Direction::Left) => (149 - board.row, 50, Direction::Right),
        // 5
        (2, 1, Direction::Right) => (149 - board.row, 149, Direction::Left),
        (2, 1, Direction::Down) => (100 + board.col, 49, Direction::Left),
        // 6
        (3, 0, Direction::Left) => (0, board.row - 100, Direction::Down),
        (3, 0, Direction::Right) => (149, board.row - 100, Direction::Up),
        (3, 0, Direction::Down) => (0, board.col + 100, Direction::Down),
        _ => panic!()
    }
}

fn part1(board: &mut Board, moves: &Vec<Move>) -> i64 {
    make_moves(board, moves, &wrap_flat)
}

fn part2(board: &mut Board, moves: &Vec<Move>) -> i64 {
    make_moves(board, moves, &wrap_3d)
}

pub(crate) fn solve() {
    let content = read_to_string("22.txt").unwrap();
    let (mut board, moves) = parse_inputs(&content);
    println!("{}", part1(&mut board, &moves));
    println!("{}", part2(&mut board, &moves));
}