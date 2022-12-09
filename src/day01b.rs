use aoc2022::read_file;

pub fn main() {
    let file = read_file!("./inputs/input01.txt");

    let mut elfs = file.split("\r\n\r\n")
        .map(|elf| {
            elf.split("\r\n")
                .map(|cal| cal.parse::<u32>().unwrap())
                .fold(0, |a, n| a + n)
        })
        .collect::<Vec<u32>>();
    
    elfs.sort();
    elfs.reverse();

    let cals: u32 = elfs.iter()
        .take(3)
        .sum();

    println!("{cals}");
}