use aoc2022::read_file;

pub fn main() {
    let file = read_file!("./inputs/input10.txt");

    let mut sum = 0;
    let mut signals: [(bool, i32); 6] = [
        (false, 20), (false, 60), (false, 100),
        (false, 140), (false, 180), (false, 220)
    ];
    let mut prevx = -1;
    let mut cycle = 1;
    let mut x = 1;
    for l in file.lines() {
        // Loop through all signals to add
        for i in 0..6 {
            // If signal is not already added
            if !signals[i].0 {
                // Check cycle
                if cycle == signals[i].1 {
                    signals[i].0 = true;
                    sum += signals[i].1 * x
                } else if cycle > signals[i].1 {
                    signals[i].0 = true;
                    sum += signals[i].1 * prevx;
                }
            }
        }

        if l != "noop" {
            let (_, amount) = l.split_once(' ').unwrap();
            let amount = amount.parse::<i32>().unwrap();

            prevx = x;
            x += amount;
            cycle += 2;
        } else {
            prevx = x;
            cycle += 1;
        }
    }

    println!("{sum}");
}
