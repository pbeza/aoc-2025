use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_double_pattern_string(x: u64) -> bool {
    let s = x.to_string();
    let len = s.len();

    if !len.is_multiple_of(2) {
        return false;
    }

    let (first, second) = s.split_at(len / 2);
    first == second
}

fn process_ranges<R: BufRead>(reader: R) -> u64 {
    let mut sum: u64 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            continue;
        }

        for range_str in line.split(',') {
            if let Some((start_str, end_str)) = range_str.split_once('-') {
                let start: u64 = start_str.parse().expect("Invalid start number");
                let end: u64 = end_str.parse().expect("Invalid end number");

                for num in start..=end {
                    if is_double_pattern_string(num) {
                        sum += num;
                    }
                }
            }
        }
    }

    sum
}

fn main() {
    let file = File::open("inputs/day02.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let result = process_ranges(reader);
    println!("Result: {result}");
}
