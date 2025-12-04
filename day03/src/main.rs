use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_max_joltage_part1(line: &str) -> u64 {
    let bytes = line.as_bytes();
    if bytes.len() < 2 {
        return 0;
    }

    let mut max = 0;
    let mut max_suffix = bytes.last().unwrap() - b'0';

    // Scan from second-to-last to first
    for i in (0..bytes.len() - 1).rev() {
        let digit = bytes[i] - b'0';
        let joltage = (digit as u64) * 10 + (max_suffix as u64);
        max = max.max(joltage);
        max_suffix = max_suffix.max(digit);
    }

    max
}

fn find_max_joltage_part2(digits: &str) -> u64 {
    let chars: Vec<char> = digits.chars().collect();
    let n = chars.len();

    if n <= 12 {
        return digits.parse().unwrap_or(0);
    }

    // Build prefix maximum: prefix_max[i] = index of max char in [0, i]
    let mut prefix_max = vec![0; n];
    for i in 1..n {
        prefix_max[i] = if chars[i] >= chars[prefix_max[i - 1]] {
            i
        } else {
            prefix_max[i - 1]
        };
    }

    // Build suffix maximum: suffix_max[i] = index of max char in [i, n)
    let mut suffix_max = vec![n - 1; n];
    for i in (0..n - 1).rev() {
        suffix_max[i] = if chars[i] >= chars[suffix_max[i + 1]] {
            i
        } else {
            suffix_max[i + 1]
        };
    }

    let mut result: u64 = 0;
    let mut start = 0;

    for pos in 0..12 {
        let remaining_needed = 11 - pos;
        let end = n - remaining_needed - 1;

        // Find max in range [start, end]
        // Use suffix_max[start] but only consider up to end
        let mut max_char = chars[start];
        let mut max_idx = start;

        // Quick check: is the suffix max in our range?
        let suffix_candidate = suffix_max[start];
        if suffix_candidate <= end {
            max_idx = suffix_candidate;
        } else {
            // The suffix max is too far right, scan to find max in [start, end]
            for i in start..=end {
                if chars[i] > max_char {
                    max_char = chars[i];
                    max_idx = i;
                }
            }
        }

        result = result * 10 + (chars[max_idx] as u64 - '0' as u64);
        start = max_idx + 1;
    }

    result
}

fn process_input<R: BufRead, F>(reader: R, f: F) -> u64
where
    F: Fn(&str) -> u64,
{
    let mut total = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            continue;
        }
        total += f(&line);
    }

    total
}

fn read_input() -> BufReader<File> {
    let file = File::open("inputs/day03.txt").expect("Failed to open input file");
    BufReader::new(file)
}

fn main() {
    let result = process_input(read_input(), find_max_joltage_part1);
    println!("Part 1: {result}");
    let result = process_input(read_input(), find_max_joltage_part2);
    println!("Part 2: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_find_max_joltage() {
        assert_eq!(find_max_joltage_part1("987654321111111"), 98);
        assert_eq!(find_max_joltage_part1("811111111111119"), 89);
        assert_eq!(find_max_joltage_part1("234234234234278"), 78);
        assert_eq!(find_max_joltage_part1("818181911112111"), 92);
    }

    #[test]
    fn test_process_input() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let reader = Cursor::new(input);
        let result = process_input(reader, find_max_joltage_part1);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part1() {
        let file = File::open("../inputs/day03.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let result = process_input(reader, find_max_joltage_part1);
        assert_eq!(result, 17316);
    }
}
