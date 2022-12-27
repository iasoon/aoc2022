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
        (5, 1) => solutions::day05::part1(input_path),
        (5, 2) => solutions::day05::part2(input_path),
        (6, 1) => solutions::day06::part1(input_path),
        (6, 2) => solutions::day06::part2(input_path),
        (7, 1) => solutions::day07::part1(input_path),
        (7, 2) => solutions::day07::part2(input_path),
        (8, 1) => solutions::day08::part1(input_path),
        (8, 2) => solutions::day08::part2(input_path),
        (9, 1) => solutions::day09::part1(input_path),
        (9, 2) => solutions::day09::part2(input_path),
        (10, 1) => solutions::day10::part1(input_path),
        (10, 2) => solutions::day10::part2(input_path),
        (11, 1) => solutions::day11::part1(input_path),
        (11, 2) => solutions::day11::part2(input_path),
        (12, 1) => solutions::day12::part1(input_path),
        (12, 2) => solutions::day12::part2(input_path),
        (13, 1) => solutions::day13::part1(input_path),
        (13, 2) => solutions::day13::part2(input_path),
        (14, 1) => solutions::day14::part1(input_path),
        (14, 2) => solutions::day14::part2(input_path),
        (15, 1) => solutions::day15::part1(input_path),
        (15, 2) => solutions::day15::part2(input_path),
        (16, 1) => solutions::day16::part1(input_path),
        (16, 2) => solutions::day16::part2(input_path),
        (17, 1) => solutions::day17::part1(input_path),
        (17, 2) => solutions::day17::part2(input_path),
        (18, 1) => solutions::day18::part1(input_path),
        (18, 2) => solutions::day18::part2(input_path),
        (19, 1) => solutions::day19::part1(input_path),
        (19, 2) => solutions::day19::part2(input_path),
        (20, 1) => solutions::day20::part1(input_path),
        (20, 2) => solutions::day20::part2(input_path),
        (21, 1) => solutions::day21::part1(input_path),
        (21, 2) => solutions::day21::part2(input_path),
        (22, 1) => solutions::day22::part1(input_path),
        (22, 2) => solutions::day22::part2(input_path),
        (23, 1) => solutions::day23::part1(input_path),
        (23, 2) => solutions::day23::part2(input_path),
        (24, 1) => solutions::day24::part1(input_path),
        (24, 2) => solutions::day24::part2(input_path),
        _ => panic!("unknown day {}", day_num),
    };
}
