mod solutions;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    // expect two arguments: day number and input file path
    let day_num = argv[1].parse::<isize>().expect("arg 1 was not a number");
    let input_path = &argv[2];
    match day_num {
        1 => solutions::day01::solve(input_path),
        _ => panic!("unknown day {}", day_num),
    };
}
