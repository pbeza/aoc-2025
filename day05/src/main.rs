use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_fresh_ingredients<R: BufRead>(reader: R) -> usize {
    let mut lines = reader.lines().map(|l| l.expect("Failed to read line"));
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        if let Some((start, end)) = line.split_once('-') {
            ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        }
    }

    ranges.sort_unstable();
    ranges.dedup_by(|b, a| (b.0 <= a.1 + 1).then(|| a.1 = a.1.max(b.1)).is_some());

    lines
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|&id| {
            let pos = ranges.partition_point(|(s, _)| *s <= id);
            pos > 0 && id <= ranges[pos - 1].1
        })
        .count()
}

fn read_input() -> BufReader<File> {
    let file = File::open("inputs/day05.txt").expect("Failed to open input file");
    BufReader::new(file)
}

fn main() {
    let result = count_fresh_ingredients(read_input());
    println!("Part 1: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let reader = Cursor::new(input);
        let result = count_fresh_ingredients(reader);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1() {
        let file = File::open("../inputs/day05.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let result = count_fresh_ingredients(reader);
        assert_eq!(result, 690);
    }
}
