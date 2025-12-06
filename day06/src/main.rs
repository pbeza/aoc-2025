use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_worksheet<R: BufRead>(reader: R) -> u64 {
    let rows: Vec<Vec<&str>> = reader
        .lines()
        .map(|l| l.unwrap().leak() as &str)
        .map(|line| line.split_whitespace().collect())
        .collect();

    (0..rows[0].len())
        .filter_map(|col| {
            let mut numbers = Vec::new();
            let mut operator = None;

            for row in &rows {
                match row[col] {
                    "+" | "*" => operator = row[col].chars().next(),
                    num => {
                        if let Ok(n) = num.parse::<u64>() {
                            numbers.push(n);
                        }
                    }
                }
            }

            operator.map(|op| {
                if op == '+' {
                    numbers.iter().sum::<u64>()
                } else {
                    numbers.iter().product::<u64>()
                }
            })
        })
        .sum()
}

fn read_input() -> BufReader<File> {
    let file = File::open("../inputs/day06.txt").expect("Failed to open input file");
    BufReader::new(file)
}

fn main() {
    let result = solve_worksheet(read_input());
    println!("Part 1: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        // The example in the problem description doesn't have separator columns,
        // so let's add them to match the expected format
        let input = "123  328  51  64\n45   64   387 23\n6    98   215 314\n*    +    *   +";
        let reader = Cursor::new(input);
        let result = solve_worksheet(reader);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part1() {
        let result = solve_worksheet(read_input());
        println!("Part 1 result: {result}");
        assert_eq!(result, 6169101504608);
    }
}
