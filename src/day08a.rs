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

    let mut count = lines.len() * 4 - 4;

    for y in 1..(columns.len() - 1) {
        for x in 1..(lines.len() - 1) {
            // Check visibility on each side
            let n = lines[y][x];
            let right_visible = lines[y][x+1..].iter().all(|n2| n > *n2);
            let left_visible = lines[y][..x].iter().all(|n2| n > *n2);
            let up_visible = columns[x][..y].iter().all(|n2| n > *n2);
            let down_visible = columns[x][y+1..].iter().all(|n2| n > *n2);

            if right_visible || left_visible || up_visible || down_visible {
                count += 1;
            }
        }
    }
    println!("{count}");
}