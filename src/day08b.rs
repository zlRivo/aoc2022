use aoc2022::read_file;

pub fn main() {
    let file = read_file!("./inputs/input08.txt");

    let lines = file.lines()
        .map(|l| {
            l.bytes()
                .map(|b| b - b'0')
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    
    let mut columns: Vec<Vec<u8>> = vec![vec![]; lines[0].len()];
    for l in &lines {
        for i in 0..lines[0].len() {
            columns[i].push(l[i]);
        }
    }

    let mut highest = 1;

    for y in 1..(columns.len() - 1) {
        for x in 1..(lines.len() - 1) {
            // Count distance on each side
            let n = lines[y][x];
            let right = lines[y][x+1..].iter().position(|n2| *n2 >= n).unwrap_or_else(|| lines[y][x+1..].len() - 1);
            let left = lines[y][..x].iter().rev().position(|n2| *n2 >= n).unwrap_or_else(|| lines[y][..x].len() - 1);
            let up = columns[x][..y].iter().rev().position(|n2| *n2 >= n).unwrap_or_else(|| columns[x][..y].len() - 1);
            let down = columns[x][y+1..].iter().position(|n2| *n2 >= n).unwrap_or_else(|| columns[x][y+1..].len() - 1);

            let scenic_score = (right+1) * (left+1) * (up+1) * (down+1);
            if scenic_score > highest {
                highest = scenic_score;
            }
        }
    }
    println!("{highest}");
}