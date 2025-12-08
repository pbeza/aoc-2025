use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_splits<R: BufRead>(reader: R) -> usize {
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let (height, width) = (grid.len(), grid[0].len());

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| row.iter().position(|&c| c == 'S').map(|c| (r, c)))
        .unwrap();

    let mut splits = 0;
    let mut beams = vec![start];
    let mut visited = HashSet::new();

    while !beams.is_empty() {
        beams = beams
            .iter()
            .filter_map(|&(row, col)| {
                (row..height).find_map(|r| {
                    (grid[r][col] == '^').then(|| {
                        if visited.insert((r, col)) {
                            splits += 1;
                            [
                                (col > 0).then_some((r, col - 1)),
                                (col < width - 1).then_some((r, col + 1)),
                            ]
                            .into_iter()
                            .flatten()
                            .collect()
                        } else {
                            vec![]
                        }
                    })
                })
            })
            .flatten()
            .collect();
    }

    splits
}

fn read_input() -> BufReader<File> {
    let file = File::open("../inputs/day07.txt").expect("Failed to open input file");
    BufReader::new(file)
}

fn main() {
    let result = count_splits(read_input());
    println!("Part 1: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = ".......S.......\n\
                     ...............\n\
                     .......^.......\n\
                     ...............\n\
                     ......^.^......\n\
                     ...............\n\
                     .....^.^.^.....\n\
                     ...............\n\
                     ....^.^...^....\n\
                     ...............\n\
                     ...^.^...^.^...\n\
                     ...............\n\
                     ..^...^.....^..\n\
                     ...............\n\
                     .^.^.^.^.^...^.\n\
                     ...............";
        let reader = Cursor::new(input);
        assert_eq!(count_splits(reader), 21);
    }

    #[test]
    fn test_part1() {
        let result = count_splits(read_input());
        assert_eq!(result, 1553);
    }
}
