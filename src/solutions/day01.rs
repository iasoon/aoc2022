use std::{
    cmp::Reverse,
    fs::File,
    io::{BufRead, BufReader},
};

fn solve(input_path: &str, n_max: usize) {
    let file = File::open(input_path).expect("could not open file");
    let reader = BufReader::new(file);
    let mut elves = Vec::new();
    let mut elf_calories = 0;

    for line_result in reader.lines() {
        let line = line_result.expect("failed to read line");
        if line.len() == 0 {
            elves.push(elf_calories);
            elf_calories = 0;
        } else {
            let calories = line.parse::<usize>().expect("not a number");
            elf_calories += calories;
        }
    }

    elves.sort_by_key(|&e| Reverse(e));
    let total_calories: usize = elves.iter().take(n_max).sum();

    println!("{}", total_calories);
}

pub fn part1(input_path: &str) {
    solve(input_path, 1)
}

pub fn part2(input_path: &str) {
    solve(input_path, 3)
}
