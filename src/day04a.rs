use aoc2022::read_file;

pub fn main() {
    let file = read_file!("./inputs/input04.txt");

    let mut count = 0;

    for l in file.lines() {
        let mut sides = l.split(",").collect::<Vec<&str>>();

        let left = sides[0].split("-")
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let left_min = left[0];
        let left_max = left[1];

        let right = sides[1].split("-")
            .map(|n| n.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let right_min = right[0];
        let right_max = right[1];

        if (left_min <= right_min && left_max >= right_max)
            || (right_min <= left_min && right_max >= left_max) {
                count += 1;
        }
    }

    println!("{count}");
}