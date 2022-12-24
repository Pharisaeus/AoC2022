use std::collections::HashSet;
use std::fs::read_to_string;
use itertools::Itertools;
use regex::internal::Char;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    fn new(c: &char) -> Direction {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            &_ => panic!()
        }
    }
    fn move_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::None => (0, 0)
        }
    }

    fn move_by_time(&self, row: i32, col: i32, minute: i32) -> (i32, i32) {
        let (vr, vc) = self.move_vector();
        (row + vr * minute, col + vc * minute)
    }
}

struct Blizzard {
    direction: Direction,
    row: i32,
    col: i32,
}

impl Blizzard {
    fn new(c: &char, row: i32, col: i32) -> Blizzard {
        Blizzard {
            direction: Direction::new(c),
            row,
            col,
        }
    }
    fn position_at_time(&self, minute: i32) -> (i32, i32) {
        self.direction.move_by_time(self.row, self.col, minute)
    }
}

struct Board {
    board: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: (i32, i32),
    end: (i32, i32),
    initial_blizzards: Vec<Blizzard>,
    blizzard_positions: Vec<HashSet<(i32, i32)>>,
}

impl Board {
    fn new(content: &str) -> Board {
        let board = content.split("\n").map(|line| line.chars().collect_vec()).collect_vec();
        let width = board.get(0).unwrap().len();
        let height = board.len();
        let initial_blizzards = board.iter()
            .enumerate()
            .flat_map(|(row, columns)| columns.iter()
                .enumerate()
                .filter(|(col, &c)| (c != '.') & (c != '#'))
                .map(move |(col, c)| Blizzard::new(c, row as i32, col as i32))
            )
            .collect_vec();
        let blizzard_positions: Vec<HashSet<(i32, i32)>> = vec![initial_blizzards.iter().map(|b| (b.row, b.col)).collect()];
        let start = (0, 1);
        let end = (height as i32 - 1, width as i32 - 2);
        Board {
            board,
            width,
            height,
            start,
            end,
            initial_blizzards,
            blizzard_positions,
        }
    }

    fn to_string_at(&self, minute: usize) -> String {
        let positions = self.blizzard_positions.get(minute).unwrap();
        let mut res = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                if (row == 0) | (row == self.height - 1) | (col == 0) | (col == self.width - 1) {
                    if self.is_wall(row, col) {
                        res += "#";
                    } else {
                        res += ".";
                    }
                } else if positions.contains(&(row as i32, col as i32)) {
                    res += "b"
                } else {
                    res += "."
                }
            }
            res += "\n";
        }
        res
    }

    fn is_wall(&self, row: usize, col: usize) -> bool {
        *self.board.get(row).unwrap().get(col).unwrap() == '#'
    }

    fn progress_blizzards(&mut self) {
        let minute = self.blizzard_positions.len();
        let new_positions = self.initial_blizzards.iter()
            .map(|b| b.position_at_time(minute as i32))
            .map(|(row, col)| self.wrap_to_board(row, col))
            .collect();
        self.blizzard_positions.push(new_positions);
    }

    fn wrap_to_board(&self, row: i32, col: i32) -> (i32, i32) {
        return if ((row, col) == self.start) | ((row, col) == self.end) {
            (row, col)
        } else {
            ((row - 1).rem_euclid(self.height as i32 - 2) + 1, (col - 1).rem_euclid(self.width as i32 - 2) + 1)
        };
    }

    fn step(&self, row: i32, col: i32, direction: &Direction) -> Option<(i32, i32)> {
        let (row, col) = direction.move_by_time(row, col, 1);
        if (row >= 0) && (row < self.height as i32) && (col >= 0) && (col < self.width as i32) && !self.is_wall(row as usize, col as usize) {
            Some((row, col))
        } else {
            None
        }
    }

    fn will_be_taken(&mut self, row: i32, col: i32, minute: usize) -> bool {
        while self.blizzard_positions.len() <= minute {
            self.progress_blizzards()
        }
        self.blizzard_positions.get(minute).unwrap().contains(&(row, col))
    }

    fn bfs(&mut self, start: (i32, i32), end: (i32, i32), minute: usize) -> usize {
        let directions = [Direction::Up, Direction::Down, Direction::Left, Direction::Right, Direction::None];
        let mut visited = HashSet::new();
        visited.insert((start, minute));
        let mut to_process = vec![(start, minute)];
        while to_process.len() > 0 {
            let ((row, col), minute) = to_process.pop().unwrap();
            for direction in directions {
                let next = self.step(row, col, &direction);
                if next.is_some() {
                    let (nr, nc) = next.unwrap();
                    let next_position = ((nr, nc), minute + 1);
                    if !visited.contains(&next_position) && !self.will_be_taken(nr, nc, minute + 1) {
                        visited.insert(next_position);
                        to_process.insert(0, next_position);
                        if (nr, nc) == end {
                            return minute + 1;
                        }
                    }
                }
            }
        }
        panic!()
    }
}

fn part1(board: &mut Board) -> usize {
    board.bfs((0, 1), (board.height as i32 - 1, board.width as i32 - 2), 0)
}

fn part2(board: &mut Board) -> usize {
    let there = board.bfs(board.start, board.end, 0);
    let back = board.bfs(board.end, board.start, there);
    board.bfs(board.start, board.end, back)
}

pub(crate) fn solve() {
    let content = read_to_string("24.txt").unwrap();
    let mut board = Board::new(&content);
    println!("{}", part1(&mut board));
    println!("{}", part2(&mut board));
}