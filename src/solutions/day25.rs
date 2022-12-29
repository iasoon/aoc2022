use crate::utils::Reader;

pub fn part1(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);
    let mut sum = 0;
    while reader.has_next() {
        let span = reader.take_while(|b| b != b'\n');
        let value = decode_snafu(span);
        sum += value;
        reader.skip_lit(b"\n");
    }
    let encoded = encode_snafu(sum);
    println!("{}", std::str::from_utf8(&encoded).unwrap());
}

fn decode_snafu(bytes: &[u8]) -> usize {
    let mut value = 0;
    for b in bytes {
        let digit_value = match b {
            b'=' => 0,
            b'-' => 1,
            b'0' => 2,
            b'1' => 3,
            b'2' => 4,
            _ => panic!("invalid snafu digit"),
        };
        value *= 5;
        value += digit_value;
        value -= 2;
    }
    value
}

fn encode_snafu(mut value: usize) -> Vec<u8> {
    let mut buf = Vec::new();
    while value > 0 {
        let (carry, digit) = match value % 5 {
            0 => (0, b'0'),
            1 => (0, b'1'),
            2 => (0, b'2'),
            3 => (1, b'='),
            4 => (1, b'-'),
            _ => unreachable!(),
        };
        buf.push(digit);
        value /= 5;
        value += carry;
    }
    buf.reverse();
    buf
}

pub fn part2(input_path: &str) {
    todo!()
}
