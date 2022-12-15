use std::cmp::Ordering;

use crate::utils::Reader;

pub fn part1(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);

    let mut answer = 0;
    let mut pair_num = 0;
    loop {
        pair_num += 1;
        let (fst, snd) = read_pair(&mut reader);
        if fst.compare(&snd) != Ordering::Greater {
            answer += pair_num;
        }
        if !reader.has_next() {
            break;
        }
        reader.skip_lit(b"\n");
    }

    println!("{}", answer);
}

pub fn part2(input_path: &str) {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);

    let mut packets = Vec::new();
    while reader.has_next() {
        if reader.peek() == b'\n' {
            reader.skip_lit(b"\n");
        } else {
            let item = read_item(&mut reader);
            packets.push(item);
        }
    }

    let div_a = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let div_b = Item::List(vec![Item::List(vec![Item::Number(6)])]);
    packets.push(div_a.clone());
    packets.push(div_b.clone());

    packets.sort_by(|a, b| a.compare(b));

    let pos_a = 1 + packets
        .iter()
        .position(|p| p.compare(&div_a) == Ordering::Equal)
        .unwrap();
    let pos_b = 1 + packets
        .iter()
        .position(|p| p.compare(&div_b) == Ordering::Equal)
        .unwrap();

    println!("{}", pos_a * pos_b);
}

#[derive(Debug, Clone)]
enum Item {
    List(Vec<Item>),
    Number(usize),
}

impl Item {
    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Number(a), Item::Number(b)) => a.cmp(b),
            (Item::Number(num), list @ Item::List(_)) => {
                Item::List(vec![Item::Number(*num)]).compare(list)
            }
            (list @ Item::List(_), Item::Number(num)) => {
                list.compare(&Item::List(vec![Item::Number(*num)]))
            }
            (Item::List(fst), Item::List(snd)) => {
                for (a, b) in fst.iter().zip(snd.iter()) {
                    let ordering = a.compare(b);
                    if ordering != Ordering::Equal {
                        return ordering;
                    }
                }
                fst.len().cmp(&snd.len())
            }
        }
    }
}

fn read_pair(reader: &mut Reader) -> (Item, Item) {
    let fst = read_item(reader);
    reader.skip_lit(b"\n");
    let snd = read_item(reader);
    reader.skip_lit(b"\n");
    (fst, snd)
}

fn read_item(reader: &mut Reader) -> Item {
    if reader.peek() == b'[' {
        reader.skip_lit(b"[");
        if reader.peek() == b']' {
            reader.skip_lit(b"]");
            return Item::List(Vec::new());
        }
        let mut buf = Vec::new();
        loop {
            let item = read_item(reader);
            buf.push(item);
            if reader.peek() == b']' {
                reader.skip_lit(b"]");
                return Item::List(buf);
            }
            reader.skip_lit(b",");
        }
    } else {
        let num = reader.read_usize();
        Item::Number(num)
    }
}
