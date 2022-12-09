fn count_segments_predicate<P>(input_path: &str, predicate: P)
where
    P: Fn((Segment, Segment)) -> bool,
{
    let bytes = std::fs::read(input_path).expect("failed to read input file");

    let mut num_true = 0;

    let mut offset = 0;
    while let Some(newline_pos) = bytes[offset..].iter().position(|&b| b == b'\n') {
        let segments = parse_segments(&bytes[offset..offset + newline_pos]);
        num_true += predicate(segments) as usize;
        offset += newline_pos + 1;
    }

    println!("{}", num_true);
}

pub fn part1(input_path: &str) {
    count_segments_predicate(input_path, |(fst, snd)| {
        fst.is_inside(&snd) || snd.is_inside(&fst)
    })
}

pub fn part2(input_path: &str) {
    count_segments_predicate(input_path, |(fst, snd)| fst.overlaps(&snd))
}

struct Segment {
    start: usize,
    end: usize,
}

impl Segment {
    fn is_inside(&self, other: &Segment) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn overlaps(&self, other: &Segment) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn parse_integer(bytes: &[u8]) -> usize {
    let mut result: usize = 0;
    for b in bytes {
        result *= 10;
        result += (b - b'0') as usize;
    }
    result
}

fn parse_segment(bytes: &[u8]) -> Segment {
    let dash_pos = bytes.iter().position(|&b| b == b'-').unwrap();
    Segment {
        start: parse_integer(&bytes[..dash_pos]),
        end: parse_integer(&bytes[dash_pos + 1..]),
    }
}

fn parse_segments(bytes: &[u8]) -> (Segment, Segment) {
    let comma_pos = bytes.iter().position(|&b| b == b',').unwrap();
    let fst = parse_segment(&bytes[..comma_pos]);
    let snd = parse_segment(&bytes[comma_pos + 1..]);
    (fst, snd)
}
