use aoc2022::read_file;

use std::collections::HashMap;

pub fn main() {
    let file = read_file!("./inputs/input07.txt");
    
    // This stores all the nested directories
    // to change their size
    let mut path: Vec<String> = Vec::new();

    // Using path as string for directory sizes
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();

    for l in file.lines() {
        // Skip useless information
        if l == "$ ls" || l.starts_with("dir") {
            continue;
        }

        if l == "$ cd .." {
            path.pop();
        } else if l.starts_with("$ cd") {
            let new_dir = l.split(" ").nth(2).unwrap();
            // Add new dir to path
            path.push(new_dir.to_string());
        } else { // ls output
            let size = l.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            // Increment each directory in path
            for j in 1..(path.len() + 1) {
                let s_path = path[..j].join("/");
                *dir_sizes.entry(s_path).or_insert(0) += size;
            }
        }
    }

    let result: i128 = dir_sizes.into_iter()
        .filter(|(_, sz)| *sz <= 100000)
        .map(|(_, sz)| sz as i128)
        .sum();

    println!("{result}");
}