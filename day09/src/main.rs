use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve<R: BufRead>(reader: R) -> usize {
    let points: Vec<(i32, i32)> = reader
        .lines()
        .map(|l| {
            let parts: Vec<i32> = l.unwrap().split(',').map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();

    let mut max_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let width = ((x2 - x1).abs() + 1) as usize;
            let height = ((y2 - y1).abs() + 1) as usize;
            let area = width * height;
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("../inputs/day09.txt").expect("Failed to open input file"))
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
        let input = "7,1\n\
                     11,1\n\
                     11,7\n\
                     9,7\n\
                     9,5\n\
                     2,5\n\
                     2,3\n\
                     7,3";
        let reader = Cursor::new(input);
        assert_eq!(solve(reader), 50);
    }
}
