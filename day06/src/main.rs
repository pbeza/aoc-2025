use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_worksheet<R: BufRead>(reader: R) -> (u64, u64) {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars = line.chars().collect::<Vec<_>>();
            chars.resize(width, ' ');
            chars
        })
        .collect();

    let height = grid.len();
    let is_empty_col = |col| (0..height).all(|row| grid[row][col] == ' ');

    let mut part1_total = 0u64;
    let mut part2_total = 0u64;
    let mut col = 0;

    while col < width {
        if is_empty_col(col) {
            col += 1;
            continue;
        }

        let start = col;
        while col < width && !is_empty_col(col) {
            col += 1;
        }

        let mut part1_numbers = Vec::new();
        let mut operator = None;

        for row in 0..height {
            let text: String = (start..col).map(|c| grid[row][c]).collect();
            match text.trim() {
                "+" | "*" => operator = text.trim().chars().next(),
                t => {
                    if let Ok(n) = t.parse::<u64>() {
                        part1_numbers.push(n);
                    }
                }
            }
        }

        let part2_numbers: Vec<u64> = (start..col)
            .rev()
            .filter_map(|c| {
                let digits: String = (0..height)
                    .filter_map(|row| {
                        let ch = grid[row][c];
                        ch.is_ascii_digit().then_some(ch)
                    })
                    .collect();
                digits.parse().ok()
            })
            .collect();

        if let Some(op) = operator {
            part1_total += if op == '+' {
                part1_numbers.iter().sum::<u64>()
            } else {
                part1_numbers.iter().product::<u64>()
            };
            part2_total += if op == '+' {
                part2_numbers.iter().sum::<u64>()
            } else {
                part2_numbers.iter().product::<u64>()
            };
        }
    }

    (part1_total, part2_total)
}

fn read_input() -> BufReader<File> {
    let file = File::open("../inputs/day06.txt").expect("Failed to open input file");
    BufReader::new(file)
}

fn main() {
    let (part1, part2) = solve_worksheet(read_input());
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "123  328  51  64 \n 45  64  387  23 \n  6  98  215  314\n*    +    *    +  ";
        let reader = Cursor::new(input);
        let (part1, part2) = solve_worksheet(reader);
        assert_eq!(part1, 4277556);
        assert_eq!(part2, 3263827);
    }

    #[test]
    fn test_part1() {
        let (part1, part2) = solve_worksheet(read_input());
        println!("Part 1: {part1}");
        println!("Part 2: {part2}");
        assert_eq!(part1, 6169101504608);
    }

    #[test]
    fn test_part2() {
        let (_part1, part2) = solve_worksheet(read_input());
        println!("Part 2: {part2}");
        assert_eq!(part2, 10442199710797);
    }
}
