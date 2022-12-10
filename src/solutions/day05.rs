use crate::utils::Reader;

struct Move {
    num: usize,
    from: usize,
    to: usize,
}

type Stacks = Vec<Vec<u8>>;

fn run_crane<F>(input_path: &str, crane_fn: F)
where
    F: Fn(&mut Stacks, Move),
{
    let bytes = std::fs::read(input_path).unwrap();

    let line_length = 1 + bytes.iter().position(|&b| b == b'\n').unwrap();
    let num_stacks = line_length / 4;

    let mut stacks = vec![Vec::new(); num_stacks];

    let mut line_start = 0;
    while bytes[line_start + 1] != b'1' {
        // read a line
        for i in 0..num_stacks {
            let chr = bytes[line_start + 4 * i + 1];
            if chr != b' ' {
                stacks[i].push(chr);
            }
        }

        line_start += line_length;
    }
    // we are at the label line; skip it
    let mut pos = line_start + line_length;
    pos += 1; // skip empty line

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let mut reader = Reader::from_bytes(&bytes[pos..]);
    while reader.has_next() {
        reader.skip_lit(b"move ");
        let num = reader.read_delimited_usize(b' ');
        reader.skip_lit(b"from ");
        let from = reader.read_delimited_usize(b' ') - 1;
        reader.skip_lit(b"to ");
        let to = reader.read_delimited_usize(b'\n') - 1;

        crane_fn(&mut stacks, Move { num, from, to });
    }

    let code_vec: Vec<u8> = stacks.iter().map(|s| s[s.len() - 1]).collect();
    let code = std::str::from_utf8(&code_vec).unwrap();
    println!("{}", code);
}

pub fn part1(input_path: &str) {
    run_crane(input_path, |stacks, Move { num, from, to }| {
        for _ in 0..num {
            let b = stacks[from].pop().unwrap();
            stacks[to].push(b);
        }
    })
}

pub fn part2(input_path: &str) {
    run_crane(input_path, |stacks, Move { num, from, to }| {
        let new_from_len = stacks[from].len() - num;
        for i in 0..num {
            let b = stacks[from][new_from_len + i];
            stacks[to].push(b);
        }
        stacks[from].truncate(new_from_len);
    })
}
