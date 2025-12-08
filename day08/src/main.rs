use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px != py {
            let (small, large) = if self.size[px] < self.size[py] {
                (px, py)
            } else {
                (py, px)
            };
            self.parent[small] = large;
            self.size[large] += self.size[small];
        }
    }

    fn top3_sizes(&mut self) -> usize {
        let mut sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.parent.len() {
            *sizes.entry(self.find(i)).or_insert(0) += 1;
        }
        let mut v: Vec<_> = sizes.values().copied().collect();
        v.sort_unstable_by(|a, b| b.cmp(a));
        v.iter().take(3).product()
    }
}

fn solve<R: BufRead>(reader: R, connections: usize) -> usize {
    let points: Vec<(i32, i32, i32)> = reader
        .lines()
        .map(|l| {
            let v: Vec<i32> = l.unwrap().split(',').map(|s| s.parse().unwrap()).collect();
            (v[0], v[1], v[2])
        })
        .collect();

    let n = points.len();
    let mut edges = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1, z1) = points[i];
            let (x2, y2, z2) = points[j];
            let dx = (x2 - x1) as f64;
            let dy = (y2 - y1) as f64;
            let dz = (z2 - z1) as f64;
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            edges.push((dist, i, j));
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut uf = UnionFind::new(n);
    edges
        .iter()
        .take(connections)
        .for_each(|(_, i, j)| uf.union(*i, *j));

    uf.top3_sizes()
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("../inputs/day08.txt").expect("Failed to open input file"))
}

fn main() {
    println!("Part 1: {}", solve(read_input(), 1000));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_example() {
        let input = "162,817,812\n\
                     57,618,57\n\
                     906,360,560\n\
                     592,479,940\n\
                     352,342,300\n\
                     466,668,158\n\
                     542,29,236\n\
                     431,825,988\n\
                     739,650,466\n\
                     52,470,668\n\
                     216,146,977\n\
                     819,987,18\n\
                     117,168,530\n\
                     805,96,715\n\
                     346,949,466\n\
                     970,615,88\n\
                     941,993,340\n\
                     862,61,35\n\
                     984,92,344\n\
                     425,690,689";
        let reader = Cursor::new(input);
        assert_eq!(solve(reader, 10), 40);
    }

    #[test]
    fn test_part1() {
        let result = solve(read_input(), 1000);
        assert_eq!(result, 131150);
    }
}
