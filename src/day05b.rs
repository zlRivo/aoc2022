use aoc2022::read_file;

use itertools::Itertools;

pub fn main() {
    let file = read_file!("./inputs/input05.txt");

    /* ---------
    PARSING
    --------- */

    // Get all lines
    let lines = file.as_str().lines();

    // Find a way to loop until the row are the numbers
    let stack_lines = lines
        .clone()
        .take_while(|l| l.contains("["))
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let stack_nums_line = lines
        .clone()
        .skip_while(|l| l.contains("["))
        .next()
        .unwrap();

    // Find the index of each number in the row
    let nums_idx = stack_nums_line.chars().positions(|c| c != ' ').collect::<Vec<usize>>();

    // Create n (biggest number) stacks (vectors) to store each letter
    let mut stacks: Vec<Vec<char>> = vec![vec![]; nums_idx.len()];

    // For each number, get all the letters before current row with the same index in the line
    for l in stack_lines.iter().rev() {
        // Get all bytes in current line
        let chars: Vec<char> = l.chars().collect();
        // Loop each found number index
        for (i, line_idx) in nums_idx.iter().enumerate() {
            // Add to stack if c is not a space
            let c = chars[*line_idx];

            if c == ' ' { continue; }

            stacks[i].push(c);
        }
    }

    // Skip until line is blank

    /* ---------
    PROCESSING
    --------- */

    // Get all lines with move
    let op_lines = lines
        .clone()
        .skip_while(|l| !l.starts_with('m'))
        .collect::<Vec<&str>>();

    for l in op_lines {

        // Split each line whitespace
        let words = l.split_whitespace().collect::<Vec<&str>>();

        // Get match 2 (amount), get match 4 (from) get match 6 (to)
        let amount = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;

        
        // N (amount) times in , pop column from and save to a vector
        let mut letters: Vec<char> = Vec::with_capacity(amount);
        for _ in 0..amount {
            if let Some(c) = stacks[from].pop() {
                letters.push(c);
            }
        }

        // Push letters in reverse order
        for l in letters.iter().rev() {
            stacks[to].push(*l);
        }
    }

    let mut result = String::new();

    // Pop one letter from each stack and add to a string
    for mut s in stacks {
        if let Some(c) = s.pop() {
            result.push(c);
        }
    }

    // Print the string
    println!("{result}");
}