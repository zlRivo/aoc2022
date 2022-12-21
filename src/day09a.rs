use aoc2022::read_file;

use std::collections::HashSet;

pub fn main() {
    let file = read_file!("./inputs/input09.txt");

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let start_pos: (i32, i32) = (0, 0);
    let mut head = start_pos;
    let mut tail = start_pos;

    for l in file.lines() {
        let (dir, amount) = l.split_once(' ').unwrap();
        let amount = amount.parse::<i32>().unwrap();

        for _ in 0..amount {
            match dir {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 -= 1,
                "D" => head.1 += 1,
                _ => unreachable!()
            }
            let diff_x = head.0 - tail.0;
            let diff_y = head.1 - tail.1;

            // If the tail is too far
            if diff_x.abs() > 1 || diff_y.abs() > 1 {
                // Move it in the right direction
                tail.0 += diff_x.signum();
                tail.1 += diff_y.signum();
            }

            visited.insert(tail);
        }
    }

    println!("{}", visited.iter().count())
}