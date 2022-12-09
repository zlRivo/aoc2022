use aoc2022::read_file;

use std::collections::HashSet;

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

    let mut sum = 0;

    for l in file.lines() {
        // Get length to split string in half
        let s_len = l.len() / 2;

        let mut left: HashSet<char> = HashSet::new();
        let mut right: HashSet<char> = HashSet::new();

        // Loop left part
        for c in (&l[..s_len]).chars() {
            left.insert(c);
        }

        // Loop right part
        for c in (&l[s_len..]).chars() {
            right.insert(c);
        }

        // Find unique letter
        for c in ('a'..='z').chain('A'..='Z') {
            if left.contains(&c) && right.contains(&c) {
                sum += get_priority(c);
                continue;
            }
        }
    }

    println!("{sum}");
}