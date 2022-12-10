mod solutions;
mod utils;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    // expect three arguments: day number, part number, input file path
    let day_num = argv[1].parse::<isize>().expect("arg 1 was not a number");
    let part = argv[2].parse::<isize>().expect("arg 2 was not a number");
    let input_path = &argv[3];
    match (day_num, part) {
        (1, 1) => solutions::day01::part1(input_path),
        (1, 2) => solutions::day01::part2(input_path),
        (2, 1) => solutions::day02::part1(input_path),
        (2, 2) => solutions::day02::part2(input_path),
        (3, 1) => solutions::day03::part1(input_path),
        (3, 2) => solutions::day03::part2(input_path),
        (4, 1) => solutions::day04::part1(input_path),
        (4, 2) => solutions::day04::part2(input_path),
        _ => panic!("unknown day {}", day_num),
    };
}
