use aoc2022::read_file;

use std::collections::HashSet;

pub fn main() {
    let file = read_file!("./inputs/input09.txt");

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let start_pos: (i32, i32) = (0, 0);
    let mut knots = [start_pos; 10];

    for l in file.lines() {
        let (dir, amount) = l.split_once(' ').unwrap();
        let amount = amount.parse::<i32>().unwrap();

        for _ in 0..amount {
            match dir {
                "R" => knots[9].0 += 1,
                "L" => knots[9].0 -= 1,
                "U" => knots[9].1 -= 1,
                "D" => knots[9].1 += 1,
                _ => unreachable!()
            }

            // Update each knot
            for i in (0..9).rev() {
                let diff_x = knots[i + 1].0 - knots[i].0;
                let diff_y = knots[i + 1].1 - knots[i].1;

                // If the knot is too far
                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    // Move it in the right direction
                    knots[i].0 += diff_x.signum();
                    knots[i].1 += diff_y.signum();
                }
            }

            // Push last knot
            visited.insert(knots[0]);
        }
    }

    println!("{}", visited.iter().count())
}