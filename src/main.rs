mod jobs;
mod day01a;
mod day01b;
mod day02a;
mod day02b;
mod day03a;
mod day03b;
mod day04a;
mod day04b;
mod day05a;
mod day05b;
mod day06a;
mod day06b;
mod day07a;
mod day07b;
mod day08a;
mod day08b;
mod day09a;
mod day09b;

fn main() {
    for (j, name) in jobs::jobs().iter() {
        print!("{name}: ");
        j();
    }
}