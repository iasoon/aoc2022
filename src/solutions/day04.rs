use crate::utils::Reader;

fn count_segments_predicate<P>(input_path: &str, predicate: P)
where
    P: Fn((Segment, Segment)) -> bool,
{
    let bytes = std::fs::read(input_path).expect("failed to read input file");
    let mut reader = Reader::from_bytes(&bytes);

    let mut num_true = 0;
    while reader.has_next() {
        let segments = parse_segments(&mut reader);
        num_true += predicate(segments) as usize;
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

fn parse_segments(reader: &mut Reader) -> (Segment, Segment) {
    let a = reader.read_delimited_usize(b'-');
    let b = reader.read_delimited_usize(b',');
    let c = reader.read_delimited_usize(b'-');
    let d = reader.read_delimited_usize(b'\n');
    (Segment { start: a, end: b }, Segment { start: c, end: d })
}
