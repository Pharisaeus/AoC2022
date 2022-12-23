use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;


#[derive(Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn next_direction(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::E,
            Direction::E => Direction::N
        }
    }

    fn check_positions(&self) -> Vec<(i32, i32)> {
        match self {
            Direction::N => vec![(-1, 0), (-1, -1), (-1, 1)],
            Direction::S => vec![(1, 0), (1, -1), (1, 1)],
            Direction::E => vec![(0, 1), (-1, 1), (1, 1)],
            Direction::W => vec![(0, -1), (-1, -1), (1, -1)],
        }
    }

    fn move_vector(&self) -> (i32, i32) {
        match self {
            Direction::N => (-1, 0),
            Direction::S => (1, 0),
            Direction::E => (0, 1),
            Direction::W => (0, -1)
        }
    }
}

struct Board {
    positions: HashSet<(i32, i32)>,
    direction: Direction,
}

impl Board {
    fn new(content: &str) -> Board {
        let positions: HashSet<(i32, i32)> = content.split("\n")
            .enumerate()
            .flat_map(|(x, line)| line.chars()
                .enumerate()
                .filter(|(y, c)| *c == '#')
                .map(move |(y, c)| (x as i32, y as i32))
            ).collect();
        Board {
            positions,
            direction: Direction::N,
        }
    }
    fn bounds(&self) -> (i32, i32, i32, i32) {
        let sx = self.positions.iter().map(|(x, y)| x).min().unwrap();
        let ex = self.positions.iter().map(|(x, y)| x).max().unwrap();
        let sy = self.positions.iter().map(|(x, y)| y).min().unwrap();
        let ey = self.positions.iter().map(|(x, y)| y).max().unwrap();
        (*sx, *ex, *sy, *ey)
    }

    fn is_fixed_position(&self, (x, y): &(i32, i32)) -> bool {
        let mut direction = Direction::N;
        for _ in 0..4 {
            for (vx, vy) in direction.check_positions() {
                if self.positions.contains(&(x + vx, y + vy)) {
                    return false;
                }
            }
            direction = direction.next_direction();
        }
        true
    }

    fn score(&self) -> i32 {
        let (sx, ex, sy, ey) = self.bounds();
        (ex - sx + 1) * (ey - sy + 1) - self.positions.len() as i32
    }

    fn nothing_moves(&self) -> bool {
        self.positions
            .iter()
            .all(|pos| self.is_fixed_position(pos))
    }

    fn can_elf_move_in_direction(&self, (x, y): &(i32, i32), direction: &Direction) -> bool {
        for (vx, vy) in direction.check_positions() {
            if self.positions.contains(&(x + vx, y + vy)) {
                return false;
            }
        }
        return true;
    }

    fn move_one_elf(&self, (x, y): (i32, i32)) -> (i32, i32) {
        if self.is_fixed_position(&(x, y)) {
            return (x, y);
        }
        let mut current_elf_direction = self.direction;
        for _ in 0..4 {
            if self.can_elf_move_in_direction(&(x, y), &current_elf_direction) {
                let (vx, vy) = current_elf_direction.move_vector();
                let (nx, ny) = (x + vx, y + vy);
                return (nx, ny);
            }
            current_elf_direction = current_elf_direction.next_direction();
        }
        return (x, y);
    }

    fn play_round(&mut self) -> Board {
        let mut taken: HashMap<(i32, i32), i32> = HashMap::new();
        let mut moves: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        for &(x, y) in self.positions.iter() {
            let (nx, ny) = self.move_one_elf((x, y));
            moves.insert((x, y), (nx, ny));
            taken.insert((nx, ny), 1 + *taken.get(&(nx, ny)).unwrap_or(&0));
        }
        let mut positions = HashSet::new();
        for pos in self.positions.iter() {
            let target_pos = moves.get(pos).unwrap();
            if *taken.get(target_pos).unwrap() == 1 {
                positions.insert(*target_pos);
            } else {
                positions.insert(*pos);
            }
        }
        Board {
            positions,
            direction: self.direction.next_direction(),
        }
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let (sx, ex, sy, ey) = self.bounds();
        let mut res = String::new();
        for x in sx - 1..ex + 2 {
            for y in sy - 1..ey + 2 {
                if self.positions.contains(&(x, y)) {
                    res += "#";
                } else {
                    res += ".";
                }
            }
            res += "\n"
        }
        res
    }
}

fn part1(mut initial_board: Board) -> i32 {
    let mut board = initial_board;
    for _ in 0..10 {
        board = board.play_round()
    }
    board.score()
}

fn part2(mut initial_board: Board) -> i32 {
    let mut round = 1;
    let mut board = initial_board;
    loop {
        if board.nothing_moves() {
            return round;
        } else {
            round += 1;
            board = board.play_round();
        }
    }
}

pub(crate) fn solve() {
    let content = read_to_string("23.txt").unwrap();
    println!("{}", part1(Board::new(&content)));
    println!("{}", part2(Board::new(&content)));
}