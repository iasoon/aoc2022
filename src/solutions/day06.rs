fn find_marker(input_path: &str, n: usize) {
    let bytes = std::fs::read(input_path).unwrap();

    let mut pos = 0;
    'outer: while pos + n < bytes.len() {
        for i in (pos..pos + n).rev() {
            for j in (pos..i).rev() {
                if bytes[i] == bytes[j] {
                    // j can never be part of the marker
                    pos = j + 1;
                    continue 'outer;
                }
            }
        }
        // checks passed; this is marker
        println!("{}", pos + n);
        return;
    }

    println!("No marker found");
}

pub fn part1(input_path: &str) {
    find_marker(input_path, 4);
}

pub fn part2(input_path: &str) {
    find_marker(input_path, 14);
}
