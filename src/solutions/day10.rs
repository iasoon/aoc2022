use crate::utils::Reader;

pub fn part1(input_path: &str) {
    let register_value_log = calc_register_value_log(input_path, 220);
    let answer: isize = [20usize, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|i| i as isize * register_value_log[i - 1])
        .sum();
    println!("{}", answer);
}

pub fn part2(input_path: &str) {
    let register_value_log = calc_register_value_log(input_path, 240);
    //  additional 6 for newlines
    let crt: String = register_value_log
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            let pos = i as isize % 40;
            if x - 1 <= pos && pos <= x + 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect();
    for i in 0..6 {
        println!("{}", &crt[40 * i..40 * (i + 1)]);
    }
}

fn calc_register_value_log(input_path: &str, length: usize) -> Vec<isize> {
    let bytes = std::fs::read(input_path).unwrap();

    let mut reader = Reader::from_bytes(&bytes);

    // +1 because we could overshoot by one, since the longest instruction
    // takes two cycles
    let mut register_value_log = Vec::with_capacity(length + 1);

    let mut register_value: isize = 1;

    while reader.has_next() && register_value_log.len() < length {
        let instruction = read_instruction(&mut reader);

        match instruction {
            Instruction::Noop => register_value_log.push(register_value),
            Instruction::Addx(num) => {
                register_value_log.push(register_value);
                register_value_log.push(register_value);
                register_value += num;
            }
        }
    }

    register_value_log
}

enum Instruction {
    Addx(isize),
    Noop,
}

fn read_instruction(reader: &mut Reader) -> Instruction {
    match reader.peek() {
        b'n' => {
            reader.skip_lit(b"noop\n");
            Instruction::Noop
        }
        b'a' => {
            reader.skip_lit(b"addx ");
            let num = reader.read_isize();
            reader.skip_lit(b"\n");
            Instruction::Addx(num)
        }
        _ => panic!("bad instruction"),
    }
}
