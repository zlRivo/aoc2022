use aoc2022::read_file;

use std::ops::Deref;

const LINES_PER_MONKEY: usize = 6;

struct Monkey {
    items: Vec<usize>,
    inspections: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    test: usize,
    target_true: usize,
    target_false: usize
}

impl Monkey {
    fn parse(input: &[&str]) -> Monkey {
        let items = input[1].split_whitespace()
            .rev()
            .take_while(|w| !w.contains(":"))
            .map(|w| w.replace(",", "").parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let test = input[3].split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let target_true = input[4].split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let target_false = input[5].split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let op_str = input[2].split_whitespace()
            .rev()
            .take(2)
            .collect::<Vec<&str>>();

        let operation: Box<dyn Fn(usize) -> usize> = match (op_str[1], op_str[0]) {
            ("+", "old") => { Box::new(|n: usize| n + n) },
            ("+", num) => {
                let num = num.parse::<usize>().unwrap();
                Box::new(move |n: usize| n + num)
            },
            ("*", "old") => { Box::new(move |n: usize| n * n) },
            ("*", num) => {
                let num = num.parse::<usize>().unwrap();
                Box::new(move |n: usize| n * num)
            },
            _ => unreachable!(),
        };
        
        Monkey {
            items,
            inspections: 0,
            operation,
            test,
            target_true,
            target_false
        }
    }
}

pub fn main() {
    let file = read_file!("./inputs/input11.txt");

    let filtered = file.lines()
        .filter(|l| l.trim() != "")
        .collect::<Vec<&str>>();
    
    let chunks = filtered.chunks(LINES_PER_MONKEY)
        .collect::<Vec<&[&str]>>();
    
    // Parse monkeys
    // Also calculate the common modulus of all the monkeys
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(filtered.len() / LINES_PER_MONKEY);
    let mut modulus = 1;
    for (i, c) in chunks.iter().enumerate() {
        monkeys.push(Monkey::parse(c));
        modulus *= monkeys[i].test;
    }

    // Process rounds
    let mut round = 0;
    while round < 10000 {
        // Loop each monkey
        for i in 0..monkeys.len() {
            // Inspect all items
            while let Some(n) = monkeys[i].items.pop() {
                // Increment number of inspections
                monkeys[i].inspections += 1;

                // Apply operation
                let n = monkeys[i].operation.deref()(n) % modulus;

                // Check divisibility
                let idx = if n % monkeys[i].test == 0 {
                    monkeys[i].target_true
                } else {
                    monkeys[i].target_false
                };

                // Throw to monkey
                monkeys[idx].items.push(n);
            }
        }
        round += 1;
    }

    let mut inspections = monkeys.iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();
    
    inspections.sort_by(|a, b| b.cmp(a));

    let result = inspections[0] * inspections[1];

    println!("{result}");

}