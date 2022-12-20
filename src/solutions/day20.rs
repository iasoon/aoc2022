use crate::utils::Reader;

struct List {
    values: Vec<isize>,
    next_pointers: Vec<usize>,
    prev_pointers: Vec<usize>,
}

impl List {
    fn from_vec(values: Vec<isize>) -> Self {
        let mut prev_pointers: Vec<usize> = (0..values.len()).collect();
        prev_pointers.rotate_right(1);
        let mut next_pointers: Vec<usize> = (0..values.len()).collect();
        next_pointers.rotate_left(1);
        List {
            values,
            next_pointers,
            prev_pointers,
        }
    }

    fn mix(&mut self) {
        let List {
            ref values,
            ref mut next_pointers,
            ref mut prev_pointers,
        } = self;

        for i in 0..values.len() {
            if values[i] == 0 {
                continue;
            }
            // remove from old pos
            next_pointers[prev_pointers[i]] = next_pointers[i];
            prev_pointers[next_pointers[i]] = prev_pointers[i];

            let mut steps = values[i] % (values.len() as isize - 1);
            if steps.abs() < (values.len() / 2) as isize {
                steps -= steps.signum() * (values.len()) as isize;
            }
            let mut pos = i;
            while steps > 0 {
                pos = next_pointers[pos];
                steps -= 1;
            }
            while steps < 0 {
                pos = prev_pointers[pos];
                steps += 1;
            }
            // inserting before a value is the same as inserting after the previous value
            if values[i] < 0 {
                pos = prev_pointers[pos];
            }
            // re-insert at new pos
            next_pointers[i] = next_pointers[pos];
            next_pointers[pos] = i;
            prev_pointers[i] = pos;
            prev_pointers[next_pointers[i]] = i
        }
    }
}

fn find_coordinates(list: &List) -> isize {
    let zero_pos = list.values.iter().position(|&n| n == 0).unwrap();
    let mut sum: isize = 0;
    let mut pos = zero_pos;
    for _ in 0..3 {
        for _ in 0..1000 {
            pos = list.next_pointers[pos];
        }
        sum += list.values[pos]
    }
    sum
}

pub fn part1(input_path: &str) {
    let numbers = parse_numbers(input_path);
    let mut list = List::from_vec(numbers);
    list.mix();
    let answer = find_coordinates(&list);

    println!("{}", answer);
}

pub fn part2(input_path: &str) {
    let mut numbers = parse_numbers(input_path);
    for number in numbers.iter_mut() {
        *number *= 811589153;
    }
    let mut list = List::from_vec(numbers);
    for _ in 0..10 {
        list.mix();
    }
    let answer = find_coordinates(&list);

    println!("{}", answer);
}

fn parse_numbers(input_path: &str) -> Vec<isize> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);
    let mut numbers: Vec<isize> = Vec::new();
    while reader.has_next() {
        let num = reader.read_isize();
        numbers.push(num);
        reader.skip_lit(b"\n");
    }
    numbers
}
