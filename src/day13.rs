use std::cmp::Ordering;
use std::fs::read_to_string;
use itertools::{Itertools, sorted};

struct Entry {
    list: Option<Vec<Entry>>,
    single: Option<i32>,
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        return match self {
            Entry { list: Some(x), single: None } => {
                return format!("[{elements}]", elements = x.iter().map(|e| e.to_string()).join(","));
            }
            Entry { list: None, single: Some(x) } => x.to_string(),
            _ => panic!()
        };
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        return match (self, other) {
            (
                Entry { single: Some(x), list: None },
                Entry { single: Some(y), list: None }
            ) => x.partial_cmp(y),
            (
                Entry { single: None, list: Some(x) },
                Entry { single: None, list: Some(y) }
            ) => {
                for (a, b) in x.iter().zip(y.iter()) {
                    let c = a.partial_cmp(b).unwrap();
                    if c.is_lt() {
                        return Some(Ordering::Less);
                    } else if c.is_gt() {
                        return Some(Ordering::Greater);
                    }
                }
                return if x.len() < y.len() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                };
            }
            (
                Entry { single: Some(x), list: None },
                y
            ) => Entry { single: None, list: Some(vec![Entry { single: Some(*x), list: None }]) }.partial_cmp(y),
            (
                x,
                Entry { single: Some(y), list: None },
            ) => x.partial_cmp(&Entry { single: None, list: Some(vec![Entry { single: Some(*y), list: None }]) }),
            _ => panic!()
        };
    }
}

impl PartialEq<Self> for Entry {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (
                Entry { single: Some(x), list: None },
                Entry { single: Some(y), list: None }
            ) => x.eq(y),
            (
                Entry { single: None, list: Some(x) },
                Entry { single: None, list: Some(y) }
            ) => (x.len() == y.len()) &&
                (x.iter().zip(y.iter())
                    .all(|(a, b)| a.partial_cmp(b).map(Ordering::is_eq).unwrap_or(false))),
            _ => false
        };
    }
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn extract_groups(data: &str) -> Vec<String> {
    let mut res = vec![];
    let mut current = String::new();
    let mut nesting = 0;
    for c in data.chars() {
        if c == '[' {
            nesting += 1;
        } else if c == ']' {
            nesting -= 1;
        } else if c == ',' && nesting == 0 {
            res.push(current.to_string());
            current = String::new();
        }
        if current.len() > 0 || c != ',' {
            current += c.to_string().as_str();
        }
    }
    if current.len() > 0 {
        res.push(current.to_string());
    }
    res
}

fn parse_entry(value: &str) -> Entry {
    return if value.starts_with("[") {
        let stripped = &value[1..value.len() - 1];
        let groups = extract_groups(stripped);
        Entry {
            single: None,
            list: Some(groups.iter().map(|x| parse_entry(x)).collect_vec()),
        }
    } else {
        Entry {
            list: None,
            single: Some(value.parse().unwrap()),
        }
    };
}

fn parse_set(set: &str) -> (Entry, Entry) {
    let (first, second) = set.split_once("\n").unwrap();
    (parse_entry(first), parse_entry(second))
}

fn parse(data: &str) -> Vec<(Entry, Entry)> {
    data.split("\n\n")
        .map(parse_set)
        .collect_vec()
}

fn is_good_order(pair: &(Entry, Entry)) -> bool {
    let (first, second) = pair;
    first < second
}

fn part1(entries: &Vec<(Entry, Entry)>) -> i32 {
    (0..entries.len())
        .filter(|&index| is_good_order(entries.get(index).unwrap()))
        .map(|index| index as i32 + 1)
        .sum()
}

fn part2(entries: &Vec<(Entry, Entry)>) -> i32 {
    let left = entries.iter().map(|(x, _)| x).collect_vec();
    let right = entries.iter().map(|(_, x)| x).collect_vec();
    let div1 = parse_entry("[[2]]");
    let div2 = parse_entry("[[6]]");
    let divs = vec![&div1, &div2];
    let all_entries = divs.iter().chain(left.iter()).chain(right.iter()).collect_vec();
    let sorted_entries = sorted(all_entries).collect_vec();
    let first = sorted_entries.binary_search(&&&div1).unwrap();
    let second = sorted_entries.binary_search(&&&div2).unwrap();
    (first + 1) as i32 * (second + 1) as i32
}

pub(crate) fn solve() {
    let content = read_to_string("13.txt").unwrap();
    let entries = parse(&content);
    println!("{}", part1(&entries));
    println!("{}", part2(&entries));
}