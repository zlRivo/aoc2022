use aoc2022::read_file;

use std::collections::HashSet;

use itertools::Itertools;

// Not the best code, could be optimised

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => return ((c as u8) - ('a' as u8) + 1).into(),
        'A'..='Z' => return ((c as u8) - ('A' as u8) + 27).into(),
        _ => 0
    }
}

pub fn main() {
    let file = read_file!("./inputs/input03.txt");

    let mut sum: usize = 0;

    // Cut input into chunks
    let groups = file.lines().chunks(3);

    for g in groups.into_iter() {

        let mut all_chars = vec![];

        // Get all characters from all lines in group
        for l in g {
            let mut chars: HashSet<char> = HashSet::new();
            
            for c in l.chars() {
                chars.insert(c);
            }

            all_chars.push(chars);
        }

        // Find unique letter
        for c in ('a'..='z').chain('A'..='Z') {
            if all_chars.iter().all(|chars| chars.contains(&c)) {
                sum += get_priority(c) as usize;
                continue;
            }
        }
    }

    println!("{sum}");

}