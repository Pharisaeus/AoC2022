use std::fs::read_to_string;
use itertools::Itertools;

struct Range {
    start: i32,
    stop: i32,
}

impl Range {
    fn new(entry: &str) -> Range {
        let (x, y) = entry.split_once("-").unwrap();
        Range {
            start: x.parse().unwrap(),
            stop: y.parse().unwrap(),
        }
    }
    fn fully_contains(&self, other: &Range) -> bool {
        (self.start <= other.start) & (self.stop >= other.stop)
    }

    fn overlaps_left(&self, other: &Range) -> bool {
        (self.start <= other.start) & (self.stop >= other.start)
    }
}

struct ElfPair {
    first: Range,
    second: Range,
}

impl ElfPair {
    fn new(first: Range, second: Range) -> ElfPair {
        ElfPair {
            first,
            second,
        }
    }
    fn is_fully_overlapped(&self) -> bool {
        self.first.fully_contains(&self.second) | self.second.fully_contains(&self.first)
    }

    fn overlaps(&self) -> bool {
        self.first.overlaps_left(&self.second) | self.second.overlaps_left(&self.first)
    }
}

fn part1(elfs: &Vec<ElfPair>) -> usize {
    elfs.iter()
        .filter(|pair| pair.is_fully_overlapped())
        .count()
}

fn part2(elfs: &Vec<ElfPair>) -> usize {
    elfs.iter()
        .filter(|pair| pair.overlaps())
        .count()
}


pub(crate) fn solve() {
    let content = read_to_string("4.txt").unwrap();
    let elfs = content
        .split("\n")
        .map(|line| line.split_once(",").unwrap())
        .map(|(elf1, elf2)| (Range::new(elf1), Range::new(elf2)))
        .map(|(elf1, elf2)| ElfPair::new(elf1, elf2))
        .collect_vec();
    println!("{}", part1(&elfs));
    println!("{}", part2(&elfs));
}