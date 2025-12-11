use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: &str) -> (Vec<bool>, Vec<Vec<usize>>) {
    let target_start = line.find('[').unwrap() + 1;
    let target_end = line.find(']').unwrap();
    let target: Vec<bool> = line[target_start..target_end]
        .chars()
        .map(|c| c == '#')
        .collect();

    let mut buttons = Vec::new();
    let mut in_parens = false;
    let mut current = String::new();

    for c in line[target_end + 1..].chars() {
        match c {
            '(' => {
                in_parens = true;
                current.clear();
            }
            ')' => {
                in_parens = false;
                if !current.is_empty() {
                    buttons.push(
                        current
                            .split(',')
                            .map(|s| s.trim().parse().unwrap())
                            .collect(),
                    );
                }
            }
            '{' => break,
            _ if in_parens => current.push(c),
            _ => {}
        }
    }

    (target, buttons)
}

fn solve_machine(target: &[bool], buttons: &[Vec<usize>]) -> usize {
    let (n_lights, n_buttons) = (target.len(), buttons.len());

    // Build coefficient matrix for system of linear equations over GF(2)
    let mut matrix: Vec<(u64, bool)> = (0..n_lights)
        .map(|light| {
            let bits = buttons
                .iter()
                .enumerate()
                .filter(|(_, btn)| btn.contains(&light))
                .fold(0u64, |acc, (idx, _)| acc | (1u64 << idx));
            (bits, target[light])
        })
        .collect();

    // Gaussian elimination to reduced row echelon form
    let mut pivot_col = vec![None; n_lights];
    let mut next_row = 0;

    for col in 0..n_buttons {
        // Find pivot and eliminate
        if let Some(pivot) = (next_row..n_lights).find(|&r| matrix[r].0 & (1u64 << col) != 0) {
            matrix.swap(next_row, pivot);
            pivot_col[next_row] = Some(col);

            // Eliminate all other rows
            for row in 0..n_lights {
                if row != next_row && (matrix[row].0 & (1u64 << col) != 0) {
                    matrix[row].0 ^= matrix[next_row].0;
                    matrix[row].1 ^= matrix[next_row].1;
                }
            }
            next_row += 1;
        }
    }

    // Check if system has solution
    if matrix[next_row..].iter().any(|&(_, b)| b) {
        return usize::MAX;
    }

    // Identify free variables (buttons we can choose to press or not)
    let mut is_pivot = vec![false; n_buttons];
    pivot_col
        .iter()
        .flatten()
        .for_each(|&col| is_pivot[col] = true);
    let free_vars: Vec<_> = (0..n_buttons).filter(|&i| !is_pivot[i]).collect();

    // Try all combinations of free variables to minimize button presses
    (0..(1u64 << free_vars.len()))
        .map(|free_mask| {
            let mut solution = vec![false; n_buttons];

            // Set free variables according to current combination
            for (i, &var) in free_vars.iter().enumerate() {
                solution[var] = (free_mask & (1u64 << i)) != 0;
            }

            // Compute dependent variables via back-substitution
            for row in 0..next_row {
                if let Some(col) = pivot_col[row] {
                    solution[col] = (0..n_buttons)
                        .filter(|&c| c != col && (matrix[row].0 & (1u64 << c) != 0))
                        .fold(matrix[row].1, |acc, c| acc ^ solution[c]);
                }
            }

            solution.iter().filter(|&&b| b).count()
        })
        .min()
        .unwrap_or(usize::MAX)
}

fn solve<R: BufRead>(reader: R) -> usize {
    reader
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            (!line.trim().is_empty()).then(|| {
                let (target, buttons) = parse_line(&line);
                solve_machine(&target, &buttons)
            })
        })
        .sum()
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("../inputs/day10.txt").expect("Failed to open input file"))
}

fn main() {
    println!("Part 1: {}", solve(read_input()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n\
                     [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n\
                     [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let reader = Cursor::new(input);
        assert_eq!(solve(reader), 7);
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(read_input()), 571);
    }
}
