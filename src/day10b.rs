use aoc2022::read_file;

const COLS: usize = 40;
const ROWS: usize = 6;
const SPRITE_WIDTH: usize = 3;

const ON: char = '#';
const OFF: char = '.';

fn get_pixel(cycle: i32, x: i32) -> char {
    let diff: i32 = ((cycle - 1) % 40) - x;
    if diff.abs() <= (SPRITE_WIDTH / 2) as i32 {
        ON
    } else {
        OFF
    }
}

pub fn main() {
    let file = read_file!("./inputs/input10.txt");

    let mut screen = ['.'; COLS * ROWS];
    let mut cycle = 1;
    let mut x = 1;
    for l in file.lines() {

        screen[(cycle - 1) as usize] = get_pixel(cycle, x);
        cycle += 1;

        if l != "noop" {
            let (_, amount) = l.split_once(' ').unwrap();
            let amount = amount.parse::<i32>().unwrap();

            
            screen[(cycle - 1) as usize] = get_pixel(cycle, x);
            cycle += 1;
            x += amount;
        }
    }
    let result = screen.chunks(40)
        .map(|l| l.iter().collect::<String>())
        .collect::<Vec<String>>();
    
    println!("");
    
    for l in result {
        println!("{l}");
    }
}
