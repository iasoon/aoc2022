use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1(input_path: &str) {
    let file = File::open(input_path).expect("could not open input file");
    let reader = BufReader::new(file);

    let mut total_score: usize = 0;
    for split_result in reader.split(b'\n') {
        let buf = split_result.expect("failed read");
        assert!(buf.len() == 3, "bad line");

        assert!(buf[1] == b' ');
        let opponent = match buf[0] {
            b'A' => Shape::Rock,
            b'B' => Shape::Paper,
            b'C' => Shape::Scissors,
            _ => panic!("unexpected byte {}", buf[0]),
        };
        let mine = match buf[2] {
            b'X' => Shape::Rock,
            b'Y' => Shape::Paper,
            b'Z' => Shape::Scissors,
            _ => panic!("unexpected byte {}", buf[2]),
        };

        let outcome = shapes_to_outcome(mine, opponent);
        total_score += outcome_score(outcome);
        total_score += shape_score(mine);
    }

    println!("score: {}", total_score);
}

pub fn part2(input_path: &str) {
    let file = File::open(input_path).expect("could not open input file");
    let reader = BufReader::new(file);

    let mut total_score: usize = 0;
    for split_result in reader.split(b'\n') {
        let buf = split_result.expect("failed read");
        assert!(buf.len() == 3, "bad line");

        assert!(buf[1] == b' ');
        let opponent = match buf[0] {
            b'A' => Shape::Rock,
            b'B' => Shape::Paper,
            b'C' => Shape::Scissors,
            _ => panic!("unexpected byte {}", buf[0]),
        };
        let outcome = match buf[2] {
            b'X' => Outcome::Loss,
            b'Y' => Outcome::Tie,
            b'Z' => Outcome::Win,
            _ => panic!("unexpected byte {}", buf[2]),
        };

        let mine = shape_for_outcome(outcome, opponent);
        total_score += outcome_score(outcome);
        total_score += shape_score(mine);
    }

    println!("score: {}", total_score);
}

const NUM_SHAPES: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn shape_to_num(shape: Shape) -> usize {
    match shape {
        Shape::Rock => 0,
        Shape::Paper => 1,
        Shape::Scissors => 2,
    }
}

fn num_to_shape(num: usize) -> Shape {
    match num {
        0 => Shape::Rock,
        1 => Shape::Paper,
        2 => Shape::Scissors,
        _ => panic!("bad shape num"),
    }
}

fn shape_score(shape: Shape) -> usize {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Tie,
    Loss,
}

fn outcome_to_num(outcome: Outcome) -> usize {
    match outcome {
        Outcome::Win => 1,
        Outcome::Tie => 0,
        Outcome::Loss => 2, // -1 mod 3
    }
}

fn num_to_outcome(num: usize) -> Outcome {
    match num {
        0 => Outcome::Tie,
        1 => Outcome::Win,
        2 => Outcome::Loss, // -1 mod 3
        _ => panic!("invalid outcome num"),
    }
}

fn outcome_score(outcome: Outcome) -> usize {
    match outcome {
        Outcome::Win => 6,
        Outcome::Tie => 3,
        Outcome::Loss => 0,
    }
}

fn shapes_to_outcome(mine: Shape, opponent: Shape) -> Outcome {
    num_to_outcome((NUM_SHAPES + shape_to_num(mine) - shape_to_num(opponent)) % NUM_SHAPES)
}

fn shape_for_outcome(outcome: Outcome, opponent: Shape) -> Shape {
    num_to_shape((shape_to_num(opponent) + outcome_to_num(outcome)) % NUM_SHAPES)
}

#[cfg(test)]
mod test {
    use super::*;
    use Outcome::*;
    use Shape::*;

    #[test]
    fn test_round_outcome_score() {
        assert_eq!(shapes_to_outcome(Rock, Scissors), Win);
        assert_eq!(shapes_to_outcome(Paper, Paper), Tie);
        assert_eq!(shapes_to_outcome(Paper, Scissors), Loss);
    }

    #[test]
    fn test_shape_for_outcome() {
        assert_eq!(shape_for_outcome(Win, Rock), Paper);
        assert_eq!(shape_for_outcome(Loss, Rock), Scissors);
        assert_eq!(shape_for_outcome(Tie, Paper), Paper);
    }
}
