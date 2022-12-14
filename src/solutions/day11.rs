use std::cmp::Reverse;

use crate::utils::Reader;

pub fn part1(input_path: &str) {
    calc_monkey_business_level(input_path, 20, Some(3));
}

pub fn part2(input_path: &str) {
    calc_monkey_business_level(input_path, 10000, None);
}

fn calc_monkey_business_level(input_path: &str, n_rounds: usize, worry_divisor: Option<usize>) {
    let mut monkeys = parse_input(input_path);

    let base: usize = monkeys.iter().map(|m| m.test.divisor).product();

    let mut num_inspections = vec![0; monkeys.len()];
    for _round in 0..n_rounds {
        for monkey_num in 0..monkeys.len() {
            num_inspections[monkey_num] += monkeys[monkey_num].items.len();
            while let Some(mut worry) = monkeys[monkey_num].items.pop() {
                worry = monkeys[monkey_num].operation.apply(worry);
                if let Some(divisor) = worry_divisor {
                    worry /= divisor;
                }
                worry %= base;
                let target_monkey = monkeys[monkey_num].test.get_target_monkey(worry);
                monkeys[target_monkey].items.push(worry);
            }
        }
    }
    num_inspections.sort_by_key(|&n| Reverse(n));
    println!("{}", num_inspections[0] * num_inspections[1]);
}

struct Monkey {
    items: Vec<usize>,
    operation: MonkeyOp,
    test: Test,
}

enum MonkeyOp {
    AddConst(usize),
    MulConst(usize),
    Square,
}

impl MonkeyOp {
    fn apply(&self, old: usize) -> usize {
        match self {
            MonkeyOp::AddConst(num) => old + num,
            MonkeyOp::MulConst(num) => old * num,
            MonkeyOp::Square => old * old,
        }
    }
}

struct Test {
    divisor: usize,
    true_target: usize,
    false_target: usize,
}

impl Test {
    fn get_target_monkey(&self, worry_level: usize) -> usize {
        if worry_level % self.divisor == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

// this struct is used only for parsing
struct Operation {
    left: Operand,
    right: Operand,
    operator: Operator,
}

enum Operand {
    Old,
    Const(usize),
}

enum Operator {
    Add,
    Mul,
}

// "compile" the monkey operation into a more efficient form,
// that can be evaluated with less branching.
impl Operation {
    fn into_monkey_op(self) -> MonkeyOp {
        use Operand::*;
        use Operator::*;

        let mut old = self.left;
        let mut other = self.right;
        if matches!(other, Old) {
            std::mem::swap(&mut old, &mut other);
        }
        assert!(matches!(old, Old));

        match (self.operator, other) {
            (Add, Const(num)) => MonkeyOp::AddConst(num),
            (Mul, Const(num)) => MonkeyOp::MulConst(num),
            (Add, Old) => MonkeyOp::MulConst(2),
            (Mul, Old) => MonkeyOp::Square,
        }
    }
}

fn parse_input(input_path: &str) -> Vec<Monkey> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);
    let mut monkeys = Vec::new();
    loop {
        let monkey = read_monkey(&mut reader);
        monkeys.push(monkey);
        if !reader.has_next() {
            break;
        }
        reader.skip_lit(b"\n");
    }
    monkeys
}

fn read_monkey(reader: &mut Reader) -> Monkey {
    reader.skip_lit(b"Monkey ");
    // TODO: this pattern is a bit of a footgun
    reader.skip_while(|c| c != b'\n');
    reader.skip_lit(b"\n");

    let items = read_items(reader);
    let operation = read_operation(reader).into_monkey_op();
    let test = read_test(reader);
    Monkey {
        items,
        operation,
        test,
    }
}

fn read_items(reader: &mut Reader) -> Vec<usize> {
    reader.skip_lit(b"  Starting items: ");
    let mut items: Vec<usize> = Vec::new();
    loop {
        let item = reader.read_usize();
        items.push(item);
        if reader.peek() == b'\n' {
            break;
        }
        reader.skip_lit(b", ");
    }
    reader.skip_lit(b"\n");
    items
}

fn read_operation(reader: &mut Reader) -> Operation {
    reader.skip_lit(b"  Operation: new = ");
    let left = read_operand(reader);
    reader.skip_lit(b" ");
    let operator = match reader.peek() {
        b'+' => Operator::Add,
        b'*' => Operator::Mul,
        c => panic!("unknown operator {}", c),
    };
    reader.skip(1);
    reader.skip_lit(b" ");
    let right = read_operand(reader);
    reader.skip_lit(b"\n");
    Operation {
        left,
        operator,
        right,
    }
}

fn read_operand(reader: &mut Reader) -> Operand {
    // TODO: can we abstract this pattern?
    if reader.peek() == b'o' {
        reader.skip_lit(b"old");
        Operand::Old
    } else {
        let num = reader.read_usize();
        Operand::Const(num)
    }
}

fn read_test(reader: &mut Reader) -> Test {
    reader.skip_lit(b"  Test: divisible by ");
    let divisor = reader.read_delimited_usize(b'\n');
    reader.skip_lit(b"    If true: throw to monkey ");
    let true_target = reader.read_delimited_usize(b'\n');
    reader.skip_lit(b"    If false: throw to monkey ");
    let false_target = reader.read_delimited_usize(b'\n');
    Test {
        divisor,
        true_target,
        false_target,
    }
}
