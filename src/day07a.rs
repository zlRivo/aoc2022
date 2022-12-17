use aoc2022::read_file;

use std::collections::HashMap;

pub fn main() {
    let file = read_file!("./inputs/test_input07.txt");

    // Filter out all ls's command and dir on ls output
    // and remove the dollar at the beginning
    let filtered = file.lines()
        .filter(|l| !l.contains("ls") && !l.contains("dir"))
        .map(|l| l.replace("$ ", ""))
        .collect::<Vec<String>>();
    
    // This stores all the nested directories
    // to change their size
    let mut stack: Vec<usize> = Vec::new();

    // Using lines indexes for directory sizes
    let mut dir_sizes: HashMap<usize, usize> = HashMap::new();

    for (i, l) in filtered.iter().enumerate() {
        if l == "cd .." {
            stack.pop();
        } else if l.starts_with("cd") {
            // Push line index
            stack.push(i);
            // Initialize dir size
            dir_sizes.entry(i).or_insert(0);
        } else { // ls output
            let size = l.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            for dir in &stack {
                *dir_sizes.get_mut(dir).unwrap() += size;
            }
        }
    }

    for (i, s) in dir_sizes {
        println!("{s}");
    }
}