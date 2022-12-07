use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use std::collections::HashSet;

pub fn part1(input_path: &str) {
    let file = File::open(input_path).expect("could not open input file");
    let reader = BufReader::new(file);

    let mut total_priority = 0;
    for line_result in reader.lines() {
        let line = line_result.expect("could not read line");
        total_priority += rucksack_priority(&line);
    }

    println!("total: {}", total_priority);
}

pub fn part2(input_path: &str) {
    let n_elves = 3;

    let file = File::open(input_path).expect("could not open input file");
    let reader = BufReader::new(file);

    let mut total_priority = 0;
    let mut item_map: HashMap<char, usize> = HashMap::new();
    for (line_num, line_result) in reader.lines().enumerate() {
        let nth_line = line_num % n_elves;

        let line = line_result.expect("could not read line");
        for c in line.chars() {
            if item_map.get(&c).unwrap_or(&0) == &nth_line {
                item_map.insert(c, nth_line + 1);
            }
        }

        // last line, process group
        if nth_line == n_elves - 1 {
            for (c, n) in item_map.drain() {
                if n == n_elves {
                    total_priority += char_priority(c);
                }
            }
        }
    }

    println!("total {}", total_priority);
}

fn rucksack_priority(rucksack_str: &str) -> usize {
    let (fst, snd) = split_compartiments(rucksack_str);

    let fst_set: HashSet<char> = HashSet::from_iter(fst.chars());
    let snd_set: HashSet<char> = HashSet::from_iter(snd.chars());

    fst_set
        .intersection(&snd_set)
        .cloned()
        .map(char_priority)
        .sum()
}

fn split_compartiments(rucksack_str: &str) -> (&str, &str) {
    let midpoint = rucksack_str.len() / 2;
    rucksack_str.split_at(midpoint)
}

fn char_priority(c: char) -> usize {
    if c.is_ascii_lowercase() {
        1 + c as usize - 'a' as usize
    } else if c.is_ascii_uppercase() {
        27 + c as usize - 'A' as usize
    } else {
        panic!("invalid rucksack item")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_priority() {
        assert_eq!(rucksack_priority("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(rucksack_priority("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 38);
        assert_eq!(rucksack_priority("PmmdzqPrVvPwwTWBwg"), 42);
    }
}
