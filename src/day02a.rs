use aoc2022::read_file;

const ROCK: &'static str = "A";
const PAPER: &'static str = "B";
const SCISSORS: &'static str = "C";

pub fn main() {
    let file = read_file!("./inputs/input02.txt");

    let mut score = 0;

    for l in file.lines() {
        let plays = l.split_whitespace()
            .map(|w| w.to_string())
            .collect::<Vec<String>>();
        
        match (plays[0].as_str(), plays[1].as_str()) {
            (ROCK, "X") => { score += 1 + 3; },
            (ROCK, "Y") => { score += 2 + 6; },
            (ROCK, "Z") => { score += 3 + 0; },

            (PAPER, "X") => { score += 1 + 0; },
            (PAPER, "Y") => { score += 2 + 3; },
            (PAPER, "Z") => { score += 3 + 6; },

            (SCISSORS, "X") => { score += 1 + 6; },
            (SCISSORS, "Y") => { score += 2 + 0; },
            (SCISSORS, "Z") => { score += 3 + 3; },

            _ => unreachable!(),
        }
    }

    println!("{score}");
}