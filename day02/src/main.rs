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

fn is_repeating_pattern_kmp(n: u64) -> bool {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    // Compute prefix-function (pi array)
    let mut pi = vec![0; len];
    for i in 1..len {
        let mut j = pi[i - 1];
        while j > 0 && bytes[i] != bytes[j] {
            j = pi[j - 1];
        }
        if bytes[i] == bytes[j] {
            j += 1;
        }
        pi[i] = j;
    }

    let longest_border = pi[len - 1];
    let pat_len = len - longest_border;

    longest_border > 0 && len.is_multiple_of(pat_len)
}

fn process_ranges<R: BufRead, F>(reader: R, predicate: F) -> u64
where
    F: Fn(u64) -> bool,
{
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
                    if predicate(num) {
                        sum += num;
                    }
                }
            }
        }
    }

    sum
}

fn read_input() -> BufReader<File> {
    let file = File::open("inputs/day02.txt").expect("Failed to open input file");
    BufReader::new(file)
}

fn main() {
    let result1 = process_ranges(read_input(), is_double_pattern_string);
    println!("Part 1: {result1}");

    let result2 = process_ranges(read_input(), is_repeating_pattern_kmp);
    println!("Part 2: {result2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_double_pattern_string() {
        assert!(is_double_pattern_string(1111));
        assert!(is_double_pattern_string(123123));
        assert!(is_double_pattern_string(9999));
        assert!(!is_double_pattern_string(123));
        assert!(!is_double_pattern_string(1234));
        assert!(!is_double_pattern_string(1122));
    }

    #[test]
    fn test_is_repeating_pattern_kmp() {
        assert!(is_repeating_pattern_kmp(1111));
        assert!(is_repeating_pattern_kmp(123123));
        assert!(is_repeating_pattern_kmp(123123123));
        assert!(is_repeating_pattern_kmp(9999));
        assert!(!is_repeating_pattern_kmp(123));
        assert!(!is_repeating_pattern_kmp(1234));
    }

    #[test]
    fn test_part1() {
        let file = File::open("../inputs/day02.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let result = process_ranges(reader, is_double_pattern_string);
        assert_eq!(result, 23701357374);
    }

    #[test]
    fn test_part2() {
        let file = File::open("../inputs/day02.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let result = process_ranges(reader, is_repeating_pattern_kmp);
        assert_eq!(result, 34284458938);
    }
}
