use std::fs::File;
use std::io::{BufRead, BufReader};

mod mod100;
use mod100::Mod100;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    steps: Mod100,
}

fn parse_line(line: &str) -> Instruction {
    let direction = match line.as_bytes()[0] {
        b'L' => Direction::Left,
        b'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    };
    let steps: u16 = line[1..].parse().expect("Invalid number");
    Instruction {
        direction,
        steps: Mod100::new(steps),
    }
}

fn process_input<R: BufRead>(reader: R) -> u32 {
    let mut sum = Mod100::new(50);
    let mut secret: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            continue;
        }
        let inst = parse_line(&line);
        match inst.direction {
            Direction::Left => sum = sum - inst.steps,
            Direction::Right => sum = sum + inst.steps,
        }
        if sum == Mod100::new(0) {
            secret += 1;
        }
    }

    secret
}

fn main() {
    let file = File::open("inputs/day01.txt").expect("Failed to open input file");
    let reader = BufReader::new(file);
    let secret = process_input(reader);
    println!("{secret}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_RESULT: u32 = 992;

    #[test]
    fn test_with_actual_input() {
        let file = File::open("../inputs/day01.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let result = process_input(reader);
        assert_eq!(result, EXPECTED_RESULT);
    }
}
