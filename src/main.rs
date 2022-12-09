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

fn main() {
    for (j, name) in jobs::jobs().iter() {
        print!("{name}: ");
        j();
    }
}