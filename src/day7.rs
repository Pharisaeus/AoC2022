use std::collections::HashMap;
use std::fs::read_to_string;
use itertools::Itertools;

struct File {
    _name: String,
    size: i32,
}

struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<String>,
}

impl Directory {
    fn get_size(&self, all_directories: &HashMap<String, Directory>) -> i32 {
        let files_size: i32 = self.files.iter().map(|f| f.size).sum();
        let subdirs_size: i32 = self.dirs.iter()
            .map(|dir_name| (self.name.clone() + "/" + dir_name))
            .map(|full_path| all_directories.get(full_path.as_str()).unwrap())
            .map(|d| d.get_size(all_directories))
            .sum();
        files_size + subdirs_size
    }
}

fn dir_up(current_dir: &String) -> String {
    let (parent, _) = current_dir.rsplit_once("/").unwrap();
    parent.to_string()
}

fn change_dir(command: &str, current_dir: &String) -> String {
    let (_, target) = command.split_once("cd ").unwrap();
    return match target {
        "/" => String::new(),
        ".." => dir_up(current_dir),
        dir_name => current_dir.to_string() + "/" + dir_name
    };
}

fn parse_listing(current_dir: &str, command: &str) -> Directory {
    let dirs = command.split("\n")
        .skip(1)
        .filter(|line| line.starts_with("dir"))
        .map(|line| line[4..].to_string())
        .collect_vec();
    let files = command.split("\n")
        .skip(1)
        .filter(|line| !line.starts_with("dir"))
        .map(|line| line.split_once(" ").unwrap())
        .map(|(size, name)| File { size: size.parse().unwrap(), _name: name.to_string() })
        .collect_vec();
    Directory {
        name: current_dir.to_string(),
        files,
        dirs,
    }
}

fn parse_commands(content: &String) -> HashMap<String, Directory> {
    let mut all_directories = HashMap::new();
    let mut current_dir: String = String::new();
    for command in content.split("\n$") {
        if command.starts_with(" ls") {
            all_directories.insert(current_dir.clone(), parse_listing(&current_dir, command));
        } else if command.contains(" cd ") {
            current_dir = change_dir(command, &current_dir);
        } else {
            panic!()
        }
    }
    all_directories
}

fn part1(all_dirs: &HashMap<String, Directory>) -> i32 {
    all_dirs.values()
        .map(|d| d.get_size(&all_dirs))
        .filter(|size| size <= &100000)
        .sum()
}

fn part2(all_dirs: &HashMap<String, Directory>) -> i32 {
    let unused = 70000000 - all_dirs.get("").unwrap().get_size(all_dirs);
    let missing = 30000000 - unused;
    all_dirs.values()
        .map(|d| d.get_size(&all_dirs))
        .filter(|size| size >= &missing)
        .min()
        .unwrap()
}

pub(crate) fn solve() {
    let content = read_to_string("7.txt").unwrap();
    let all_directories = parse_commands(&content);
    println!("{}", part1(&all_directories));
    println!("{}", part2(&all_directories));
}