use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_accessible_rolls_with<F, G>(
    grid: &mut [Vec<u8>],
    targets: &HashSet<u8>,
    f: F,
    after: G,
) -> usize
where
    F: Fn(&mut [Vec<u8>], usize, usize),
    G: Fn(&mut [Vec<u8>]),
{
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if targets.contains(&grid[r][c]) {
                let mut adjacent_count = 0;

                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;

                        if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                            if targets.contains(&grid[nr as usize][nc as usize]) {
                                adjacent_count += 1;
                            }
                        }
                    }
                }

                if adjacent_count < 4 {
                    count += 1;
                    f(grid, r, c);
                }
            }
        }
    }

    after(grid);

    count
}

fn count_accessible_rolls(grid: &mut [Vec<u8>]) -> usize {
    let targets = HashSet::from([b'@']);
    count_accessible_rolls_with(grid, &targets, |_, _, _| {}, |_| {})
}

fn count_and_remove_accessible_rolls(grid: &mut [Vec<u8>]) -> usize {
    let targets = HashSet::from([b'@', b'R']);
    let mut sum = 0;

    loop {
        let count = count_accessible_rolls_with(
            grid,
            &targets,
            |g, r, c| g[r][c] = b'R',
            |g| {
                for row in g {
                    for cell in row {
                        if *cell == b'R' {
                            *cell = b'.';
                        }
                    }
                }
            },
        );

        if count == 0 {
            return sum;
        }

        sum += count;
    }
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
    let mut grid = read_grid(read_input());
    let result = count_accessible_rolls(&mut grid);
    println!("Part 1: {result}");
    let mut grid = read_grid(read_input());
    let result = count_and_remove_accessible_rolls(&mut grid);
    println!("Part 2: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let reader = Cursor::new(input);
        let mut grid = read_grid(reader);
        let result = count_accessible_rolls(&mut grid);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part1() {
        let file = File::open("../inputs/day04.txt").expect("Failed to read input file");
        let reader = BufReader::new(file);
        let mut grid = read_grid(reader);
        let result = count_accessible_rolls(&mut grid);
        println!("Part 1 result: {result}");
    }
}
