use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    steps: u16,
}

fn parse_line(line: &str) -> Instruction {
    let direction = match line.as_bytes()[0] {
        b'L' => Direction::Left,
        b'R' => Direction::Right,
        _ => panic!("Invalid direction"),
    };
    let steps: u16 = line[1..].parse().expect("Invalid number");
    Instruction { direction, steps }
}

fn process_input<R: BufRead>(reader: R) -> u32 {
    let mut position = 50i32;
    let mut secret: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            continue;
        }
        let inst = parse_line(&line);

        let steps = inst.steps as i32;

        // Count complete laps
        secret += (steps / 100) as u32;

        // Calculate new position after partial rotation
        let delta = match inst.direction {
            Direction::Right => steps % 100,
            Direction::Left => -(steps % 100),
        };
        let new_pos = (position + delta).rem_euclid(100);

        // Count if we pass through 0 (wrap or land on 0, but not if starting at 0)
        if position != 0 {
            let wrapped = match inst.direction {
                Direction::Right => new_pos < position,
                Direction::Left => new_pos > position,
            };
            if wrapped || new_pos == 0 {
                secret += 1;
            }
        }

        position = new_pos;
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

    const EXPECTED_RESULT: u32 = 6133;

    #[test]
    fn test_with_actual_input() {
        let file = File::open("../inputs/day01.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let result = process_input(reader);
        assert_eq!(result, EXPECTED_RESULT);
    }
}
