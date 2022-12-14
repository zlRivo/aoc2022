pub fn jobs() -> &'static [(fn(), &'static str)] {
    &[
        (crate::day01a::main, "day01a"),
        (crate::day01b::main, "day01b"),
        (crate::day02a::main, "day02a"),
        (crate::day02b::main, "day02b"),
        (crate::day03a::main, "day03a"),
        (crate::day03b::main, "day03b"),
        (crate::day04a::main, "day04a"),
        (crate::day04b::main, "day04b"),
        (crate::day05a::main, "day05a"),
        (crate::day05b::main, "day05b"),
    ]
}