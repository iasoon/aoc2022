use std::collections::HashMap;

use crate::utils::Reader;

pub fn part1(input_path: &str) {
    let directories = parse_directories(input_path);
    let total_size: usize = directories
        .iter()
        .map(|d| d.total_size)
        .filter(|&s| s <= 100000)
        .sum();
    println!("{}", total_size);
}

pub fn part2(input_path: &str) {
    let directories = parse_directories(input_path);
    let disk_size = 70000000;
    let used = directories[0].total_size;
    let needed = 30000000usize.saturating_sub(disk_size - used);
    let size: usize = directories
        .iter()
        .map(|d| d.total_size)
        .filter(|&s| s >= needed)
        .min()
        .unwrap();
    println!("{}", size);
}

struct Directory {
    total_size: usize,
    parent_index: usize,
}

fn parse_directories(input_path: &str) -> Vec<Directory> {
    let bytes = std::fs::read(input_path).unwrap();
    let mut reader = Reader::from_bytes(&bytes);

    let mut directory_index: HashMap<(usize, &[u8]), usize> = HashMap::new();

    // this should always be the first command
    reader.skip_lit(b"$ cd /\n");
    // start with only the root directory
    let mut directories = vec![Directory {
        total_size: 0,
        parent_index: 0,
    }];
    let mut current_directory = 0;

    while reader.has_next() {
        reader.skip_lit(b"$ ");
        if reader.peek() == b'c' {
            reader.skip_lit(b"cd ");
            match reader.peek() {
                b'/' => {
                    reader.skip_lit(b"/\n");
                    current_directory = 0;
                }
                b'.' => {
                    reader.skip_lit(b"..\n");
                    current_directory = directories[current_directory].parent_index;
                }
                _ => {
                    let dir_name = reader.take_while(|b| b != b'\n');
                    reader.skip_lit(b"\n");

                    // check if dir already exists
                    if let Some(&dir_id) = directory_index.get(&(current_directory, dir_name)) {
                        current_directory = dir_id;
                    } else {
                        // create a new directory
                        let dir_id = directories.len();
                        let dir = Directory {
                            total_size: 0,
                            parent_index: current_directory,
                        };
                        directory_index.insert((current_directory, dir_name), dir_id);
                        directories.push(dir);
                        current_directory = dir_id;
                    }
                }
            }
        } else {
            reader.skip_lit(b"ls\n");
            // read entries
            while reader.has_next() && reader.peek() != b'$' {
                if reader.peek() == b'd' {
                    reader.skip_lit(b"dir ");
                    reader.skip_while(|b| b != b'\n');
                    reader.skip_lit(b"\n");
                } else {
                    let size = reader.read_usize();
                    directories[current_directory].total_size += size;
                    reader.skip_while(|b| b != b'\n');
                    reader.skip_lit(b"\n");
                }
            }
        }
    }

    for i in (1..directories.len()).rev() {
        let size = directories[i].total_size;
        let parent_index = directories[i].parent_index;
        directories[parent_index].total_size += size;
    }

    directories
}
