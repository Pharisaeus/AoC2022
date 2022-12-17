use std::collections::HashSet;
use std::detect::__is_feature_detected::sha;
use std::fs::read_to_string;
use itertools::Itertools;

#[derive(Clone)]
struct Block {
    bottom_left: (i64, i64),
    other_parts: Vec<(i64, i64)>,
}

impl Block {
    fn taken_spots(&self) -> Vec<(i64, i64)> {
        let (x, y) = self.bottom_left;
        self.other_parts
            .iter()
            .map(|(vx, vy)| (x + vx, y + vy))
            .collect_vec()
    }
    fn next_position(&self, direction: &char) -> Block {
        let (x, y) = self.bottom_left;
        let bottom_left;
        if *direction == '>' {
            bottom_left = (x + 1, y)
        } else if *direction == '<' {
            bottom_left = (x - 1, y)
        } else {
            bottom_left = (x, y - 1)
        }
        Block { bottom_left, other_parts: self.other_parts.clone() }
    }

    fn is_out_of_bounds(&self) -> bool {
        self.taken_spots()
            .iter()
            .any(|(x, _)| (*x < 0) | (*x > 6))
    }
}

struct Board {
    taken_spaces: HashSet<(i64, i64)>,
    highest_point: i64,
    moves: Vec<char>,
    current_move: usize,
    block_to_spawn: usize,
    added_blocks: usize,
    shapes: Vec<Vec<(i64, i64)>>,
}

impl Board {
    fn spawn_new_block(&mut self) -> Block {
        let y = self.highest_point + 4;
        let x = 2;
        let shape = self.shapes.get(self.block_to_spawn).unwrap();
        self.added_blocks += 1;
        self.block_to_spawn = self.added_blocks % (self.shapes.len());
        Block {
            bottom_left: (x, y),
            other_parts: shape.clone(),
        }
    }

    fn add_block_to_board(&mut self, block: &Block) {
        for (x, y) in block.taken_spots() {
            if y > self.highest_point {
                self.highest_point = y;
            }
            self.taken_spaces.insert((x, y));
        }
    }

    fn next_position(&mut self, block: &Block, round_number: i64) -> Option<Block> {
        return if round_number % 2 == 0 {
            let direction = self.moves.get(self.current_move).unwrap();
            self.current_move = (self.current_move + 1) % self.moves.len();
            let b = block.next_position(direction);
            if b.is_out_of_bounds() | self.collides(&b) {
                Some(block.clone())
            } else {
                Some(b)
            }
        } else {
            let direction = 'v';
            let b = block.next_position(&direction);
            if b.is_out_of_bounds() | self.collides(&b) {
                None
            } else {
                Some(b)
            }
        };
    }

    fn collides(&self, block: &Block) -> bool {
        block.taken_spots()
            .iter()
            .any(|x| self.taken_spaces.contains(x))
    }

    fn move_block(&mut self, block: &Block, round_number: i64) -> i64 {
        let mut current_block = block.clone();
        let mut round = round_number;
        loop {
            let new_block = self.next_position(&current_block, round);
            round += 1;
            match new_block {
                Some(b) => current_block = b,
                None => {
                    self.add_block_to_board(&current_block);
                    return round;
                }
            }
        }
    }
    fn add_new_block(&mut self, round_number: i64) -> i64 {
        let block = self.spawn_new_block();
        self.move_block(&block, round_number)
    }

    fn new(moves: &Vec<char>, shapes: &Vec<Vec<(i64, i64)>>) -> Board {
        let mut taken_spaces = HashSet::from_iter((0..7).map(|x| (x, 0)).collect_vec());
        Board {
            taken_spaces,
            highest_point: 0,
            moves: moves.clone(),
            current_move: 0,
            block_to_spawn: 0,
            added_blocks: 0,
            shapes: shapes.clone(),
        }
    }
}

fn part1(moves: &Vec<char>, shapes: &Vec<Vec<(i64, i64)>>) -> i64 {
    let mut board = Board::new(moves, shapes);
    let mut round_number = 0;
    for _ in 0..2022 {
        round_number = board.add_new_block(round_number)
    }
    board.highest_point
}

fn part2(moves: &Vec<char>, shapes: &Vec<Vec<(i64, i64)>>) -> i64 {
    let mut board = Board::new(moves, shapes);
    let mut round_number = 0;
    let mut seen_states = HashSet::new();
    let cycle_start;
    loop {
        seen_states.insert((board.current_move, board.block_to_spawn));
        round_number = board.add_new_block(round_number);
        if seen_states.contains(&(board.current_move, board.block_to_spawn)) {
            cycle_start = (board.current_move, board.block_to_spawn);
            break;
        }
    }
    let height_until_cycle = board.highest_point;
    let blocks_until_cycle = board.added_blocks;
    let cycle_size;
    let blocks_in_cycle;
    loop {
        round_number = board.add_new_block(round_number);
        if (board.current_move, board.block_to_spawn) == cycle_start {
            cycle_size = board.highest_point - height_until_cycle;
            blocks_in_cycle = board.added_blocks - blocks_until_cycle;
            break;
        }
    }
    let cycles_count = ((1000000000000 - blocks_until_cycle) / blocks_in_cycle) as i64;
    let left = (1000000000000 - blocks_until_cycle) % blocks_in_cycle;
    let before_left = board.highest_point;
    for _ in 0..left {
        round_number = board.add_new_block(round_number);
    }
    let left_height = board.highest_point - before_left;
    height_until_cycle + cycles_count * cycle_size + left_height
}

pub(crate) fn solve() {
    let content = read_to_string("17.txt").unwrap();
    let moves = content.chars().collect_vec();
    let shapes = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];
    println!("{}", part1(&moves, &shapes));
    println!("{}", part2(&moves, &shapes));
}