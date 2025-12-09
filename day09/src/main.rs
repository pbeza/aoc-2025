use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_points<R: BufRead>(reader: R) -> Vec<(i64, i64)> {
    reader
        .lines()
        .map(|l| {
            let parts: Vec<i64> = l.unwrap().split(',').map(|s| s.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn solve_part1<R: BufRead>(reader: R) -> i64 {
    let points = parse_points(reader);
    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            max_area = max_area.max(((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1));
        }
    }
    max_area
}

// Check if point is inside or on the boundary of a polygon
fn is_inside_or_on_polygon(point: (i64, i64), polygon: &[(i64, i64)]) -> bool {
    let (x, y) = point;

    // Check if point is a vertex or lies on any axis-aligned edge
    for i in 0..polygon.len() {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % polygon.len()];

        if (x, y) == (x1, y1)
            || (x >= x1.min(x2)
                && x <= x1.max(x2)
                && y >= y1.min(y2)
                && y <= y1.max(y2)
                && ((x1 == x2 && x == x1) || (y1 == y2 && y == y1)))
        {
            return true;
        }
    }

    // Ray casting algorithm: count edge crossings from point to infinity
    (0..polygon.len()).fold(false, |inside, i| {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % polygon.len()];
        if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1) {
            !inside
        } else {
            inside
        }
    })
}

// Check if two axis-aligned segments cross perpendicularly (not parallel or touching)
fn crosses(rect: ((i64, i64), (i64, i64)), poly: ((i64, i64), (i64, i64))) -> bool {
    let ((x1, y1), (x2, y2)) = rect;
    let ((x3, y3), (x4, y4)) = poly;

    (y1 == y2
        && x3 == x4
        && x3 > x1.min(x2)
        && x3 < x1.max(x2)
        && y1 > y3.min(y4)
        && y1 < y3.max(y4))
        || (x1 == x2
            && y3 == y4
            && x1 > x3.min(x4)
            && x1 < x3.max(x4)
            && y3 > y1.min(y2)
            && y3 < y1.max(y2))
}

fn solve_part2<R: BufRead>(reader: R) -> i64 {
    let points = parse_points(reader);
    let mut max_area = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x_lo, x_hi) = (points[i].0.min(points[j].0), points[i].0.max(points[j].0));
            let (y_lo, y_hi) = (points[i].1.min(points[j].1), points[i].1.max(points[j].1));

            let corners = [(x_lo, y_lo), (x_hi, y_lo), (x_hi, y_hi), (x_lo, y_hi)];
            if !corners.iter().all(|&c| is_inside_or_on_polygon(c, &points)) {
                continue;
            }

            let has_crossing = [
                ((x_lo, y_lo), (x_hi, y_lo)),
                ((x_hi, y_lo), (x_hi, y_hi)),
                ((x_hi, y_hi), (x_lo, y_hi)),
                ((x_lo, y_hi), (x_lo, y_lo)),
            ]
            .iter()
            .any(|&re| {
                (0..points.len()).any(|k| crosses(re, (points[k], points[(k + 1) % points.len()])))
            });

            if !has_crossing {
                max_area = max_area.max((x_hi - x_lo + 1) * (y_hi - y_lo + 1));
            }
        }
    }
    max_area
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("../inputs/day09.txt").expect("Failed to open input file"))
}

fn main() {
    println!("Part 1: {}", solve_part1(read_input()));
    println!("Part 2: {}", solve_part2(read_input()));
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
        assert_eq!(solve_part1(reader), 50);
        let reader = Cursor::new(input);
        assert_eq!(solve_part2(reader), 24);
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(read_input());
        assert_eq!(result, 1644094530);
    }
}
