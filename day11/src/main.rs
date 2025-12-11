use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input<R: BufRead>(reader: R) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        let node = parts[0].to_string();
        let outputs: Vec<String> = parts[1].split_whitespace().map(|s| s.to_string()).collect();

        graph.insert(node, outputs);
    }

    graph
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut HashSet<String>,
) -> usize {
    if current == target {
        return 1;
    }

    visited.insert(current.to_string());

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                total_paths += count_paths(graph, neighbor, target, visited);
            }
        }
    }

    visited.remove(current);
    total_paths
}

fn count_paths_dp(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &mut HashSet<String>,
    visited_mask: u8,
    required_map: &HashMap<String, u8>,
    memo: &mut HashMap<(String, u8), usize>,
) -> usize {
    let mut mask = visited_mask;
    if let Some(&bit) = required_map.get(current) {
        mask |= bit;
    }

    if current == target {
        return if mask == 3 { 1 } else { 0 };
    }

    let key = (current.to_string(), mask);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    visited.insert(current.to_string());

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                total_paths +=
                    count_paths_dp(graph, neighbor, target, visited, mask, required_map, memo);
            }
        }
    }

    visited.remove(current);
    memo.insert(key, total_paths);
    total_paths
}

fn solve_part1<R: BufRead>(reader: R) -> usize {
    let graph = parse_input(reader);
    let mut visited = HashSet::new();
    count_paths(&graph, "you", "out", &mut visited)
}

fn solve_part2<R: BufRead>(reader: R) -> usize {
    let graph = parse_input(reader);
    let mut visited = HashSet::new();
    let mut required_map = HashMap::new();
    required_map.insert("dac".to_string(), 1);
    required_map.insert("fft".to_string(), 2);
    let mut memo = HashMap::new();
    count_paths_dp(
        &graph,
        "svr",
        "out",
        &mut visited,
        0,
        &required_map,
        &mut memo,
    )
}

fn read_input() -> BufReader<File> {
    BufReader::new(File::open("../inputs/day11.txt").expect("Failed to open input file"))
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
    fn test_example_part1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let reader = Cursor::new(input);
        assert_eq!(solve_part1(reader), 5);
    }

    #[test]
    fn test_example_part2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let reader = Cursor::new(input);
        assert_eq!(solve_part2(reader), 2);
    }
}
