use aoc2022::read_file;

pub fn main() {
    let chars = read_file!("./inputs/input06.txt").chars().collect::<Vec<char>>();

    let mut result = -1;

    'outer: for i in 0..(chars.len() - 13) {

        // Letter bits
        let mut letters = 0u32;

        for j in 0..14 {
            // Go to next characters group if letter bit is already set
            if letters & (1 << (chars[i+j] as u8 - b'a')) > 0 {
                continue 'outer;
            }

            // Set letter bit
            letters |= 1 << (chars[i+j] as u8 - b'a');
        }

        result = i as i32 + 13 + 1;
        break;
    }

    println!("{result}");
}