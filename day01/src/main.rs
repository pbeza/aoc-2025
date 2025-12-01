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

fn part1<R: BufRead>(reader: R) -> u32 {
    let mut position = 50i32;
    let mut secret: u32 = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            continue;
        }
        let inst = parse_line(&line);

        let delta = match inst.direction {
            Direction::Right => inst.steps as i32,
            Direction::Left => -(inst.steps as i32),
        };
        position = (position + delta).rem_euclid(100);

        if position == 0 {
            secret += 1;
        }
    }

    secret
}

fn part2<R: BufRead>(reader: R) -> u32 {
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

fn read_input() -> BufReader<File> {
    let file = File::open("inputs/day01.txt").expect("Failed to open input file");
    BufReader::new(file)
}

fn main() {
    let result1 = part1(read_input());
    let result2 = part2(read_input());
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let file = File::open("../inputs/day01.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let result = part1(reader);
        assert_eq!(result, 992);
    }

    #[test]
    fn test_part2() {
        let file = File::open("../inputs/day01.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let result = part2(reader);
        assert_eq!(result, 6133);
    }
}
