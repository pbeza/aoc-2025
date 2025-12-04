use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_accessible_rolls(grid: &[Vec<u8>]) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == b'@' {
                // Count adjacent @ symbols (8 directions)
                let mut adjacent_count = 0;

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;

                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            if grid[nr as usize][nc as usize] == b'@' {
                                adjacent_count += 1;
                            }
                        }
                    }
                }

                if adjacent_count < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn read_grid<R: BufRead>(reader: R) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if !line.is_empty() {
            grid.push(line.into_bytes());
        }
    }

    grid
}

fn read_input() -> BufReader<File> {
    let file = File::open("inputs/day04.txt").expect("Failed to open input file");
    BufReader::new(file)
}

fn main() {
    let grid = read_grid(read_input());
    let result = count_accessible_rolls(&grid);
    println!("Part 1: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let reader = Cursor::new(input);
        let grid = read_grid(reader);
        let result = count_accessible_rolls(&grid);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part1() {
        let file = File::open("../inputs/day04.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let grid = read_grid(reader);
        let result = count_accessible_rolls(&grid);
        println!("Part 1 result: {result}");
    }
}
