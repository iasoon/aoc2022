use std::collections::HashMap;

use crate::utils::Reader;

static ROOT: &[u8] = b"root";
static HUMN: &[u8] = b"humn";

pub fn part1(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let monkeys = read_monkeys(&bytes);
    let monkey_values = forward_solve(&monkeys);

    // fill the monkey stack
    println!("{}", monkey_values[ROOT]);
}

fn forward_solve<'a>(monkey_map: &HashMap<&'a [u8], MonkeyJob<'a>>) -> HashMap<&'a [u8], isize> {
    let mut monkey_values: HashMap<&[u8], isize> = HashMap::new();
    let mut monkey_stack = Vec::new();

    let mut waiting_for: HashMap<&[u8], Vec<&[u8]>> = HashMap::new();
    for (monkey_name, monkey_job) in monkey_map.iter() {
        match monkey_job {
            MonkeyJob::YellNumber(num) => {
                monkey_values.insert(monkey_name, *num);
                monkey_stack.push(monkey_name);
            }
            MonkeyJob::YellResult { fst, snd, .. } => {
                for m in [fst, snd] {
                    waiting_for.entry(m).or_default().push(monkey_name);
                }
            }
        }
    }

    // forward solve
    while let Some(monkey_name) = monkey_stack.pop() {
        if let Some(waiting_monkeys) = waiting_for.get(monkey_name) {
            for m_name in waiting_monkeys {
                if monkey_values.get(m_name).is_some() {
                    // already done
                    continue;
                }
                let m_job = &monkey_map[m_name];
                if let MonkeyJob::YellResult {
                    operation,
                    fst,
                    snd,
                } = &m_job
                {
                    if let (Some(&a), Some(&b)) = (monkey_values.get(fst), monkey_values.get(snd)) {
                        let value = operation.apply(a, b);
                        monkey_values.insert(m_name, value);
                        monkey_stack.push(m_name);
                    }
                }
            }
        }
    }
    monkey_values
}

pub fn part2(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let mut monkeys = read_monkeys(&bytes);

    // fire root and humn, so that solving will not proceed for them
    monkeys.remove(HUMN);
    let root_job = monkeys.remove(ROOT).unwrap();

    let monkey_values = forward_solve(&monkeys);
    // all monkeys should be either solved now, or on a linear path from humn to root

    let (mut monkey_name, mut expected_value) = match root_job {
        MonkeyJob::YellResult { fst, snd, .. } => {
            if let Some(&value) = monkey_values.get(fst) {
                (snd, value)
            } else if let Some(&value) = monkey_values.get(snd) {
                (fst, value)
            } else {
                panic!("none of root children was solved");
            }
        }
        _ => panic!("root has a bad job"),
    };

    while monkey_name != HUMN {
        (monkey_name, expected_value) = match &monkeys[monkey_name] {
            MonkeyJob::YellResult {
                operation,
                fst,
                snd,
            } => {
                if let Some(&value) = monkey_values.get(fst) {
                    (snd, operation.solve_for_right(value, expected_value))
                } else if let Some(&value) = monkey_values.get(snd) {
                    (fst, operation.solve_for_left(value, expected_value))
                } else {
                    panic!("no child was solved");
                }
            }
            _ => panic!("monkey has a bad job"),
        }
    }
    println!("{}", expected_value);
}

fn read_monkeys(bytes: &[u8]) -> HashMap<&[u8], MonkeyJob> {
    let mut reader = Reader::from_bytes(bytes);
    let mut monkey_map = HashMap::new();
    while reader.has_next() {
        let monkey = read_monkey(&mut reader);
        monkey_map.insert(monkey.name, monkey.job);
    }
    monkey_map
}

fn read_monkey<'a>(reader: &mut Reader<'a>) -> Monkey<'a> {
    let name = reader.take_while(|c| c != b':');
    reader.skip_lit(b": ");
    let job = match reader.peek() {
        b'0'..=b'9' => {
            let num = reader.read_isize();
            MonkeyJob::YellNumber(num)
        }
        _ => {
            let fst = reader.take_while(|c| c != b' ');
            reader.skip_lit(b" ");
            let operation = match reader.take_byte() {
                b'+' => Operation::Add,
                b'-' => Operation::Sub,
                b'*' => Operation::Mul,
                b'/' => Operation::Div,
                c => panic!("invalid operation '{}'", c),
            };
            reader.skip_lit(b" ");
            let snd = reader.take_while(|c| c != b'\n');
            MonkeyJob::YellResult {
                operation,
                fst,
                snd,
            }
        }
    };
    reader.skip_lit(b"\n");

    Monkey { name, job }
}

struct Monkey<'a> {
    name: &'a [u8],
    job: MonkeyJob<'a>,
}

#[derive(Debug)]
enum MonkeyJob<'a> {
    YellNumber(isize),
    YellResult {
        operation: Operation,
        fst: &'a [u8],
        snd: &'a [u8],
    },
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn apply(&self, left: isize, right: isize) -> isize {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }

    fn solve_for_left(&self, right: isize, result: isize) -> isize {
        match self {
            Operation::Add => result - right,
            Operation::Sub => result + right,
            Operation::Mul => result / right,
            Operation::Div => result * right,
        }
    }

    fn solve_for_right(&self, left: isize, result: isize) -> isize {
        match self {
            Operation::Add => result - left,
            Operation::Sub => left - result,
            Operation::Mul => result / left,
            Operation::Div => left / result,
        }
    }
}
