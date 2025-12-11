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

// Topological sort using DFS
fn toposort(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    visited: &mut HashSet<String>,
    stack: &mut Vec<String>,
) {
    if visited.contains(node) {
        return;
    }
    visited.insert(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            toposort(graph, neighbor, visited, stack);
        }
    }

    stack.push(node.to_string());
}

// Count paths from source to all nodes using DP with topological sort
fn count_paths_from(graph: &HashMap<String, Vec<String>>, source: &str) -> HashMap<String, usize> {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    // Get all nodes reachable from source
    toposort(graph, source, &mut visited, &mut stack);
    stack.reverse(); // Process in topological order

    let mut counts = HashMap::new();
    counts.insert(source.to_string(), 1);

    for node in stack {
        let current_count = *counts.get(&node).unwrap_or(&0);
        if current_count == 0 {
            continue;
        }

        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                *counts.entry(neighbor.clone()).or_insert(0) += current_count;
            }
        }
    }

    counts
}

// Build reverse graph
fn reverse_graph(graph: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut rev = HashMap::new();

    for (node, neighbors) in graph {
        rev.entry(node.clone()).or_insert_with(Vec::new);
        for neighbor in neighbors {
            rev.entry(neighbor.clone())
                .or_insert_with(Vec::new)
                .push(node.clone());
        }
    }

    rev
}

// Count paths from all nodes to target using reverse graph
fn count_paths_to(graph: &HashMap<String, Vec<String>>, target: &str) -> HashMap<String, usize> {
    let rev = reverse_graph(graph);
    count_paths_from(&rev, target)
}

fn solve_part1<R: BufRead>(reader: R) -> usize {
    let graph = parse_input(reader);
    let mut visited = HashSet::new();
    count_paths(&graph, "you", "out", &mut visited)
}

fn solve_part2<R: BufRead>(reader: R) -> usize {
    let graph = parse_input(reader);

    // Helper to count paths visiting nodes A then B
    let count_via = |a: &str, b: &str| {
        let from_start = count_paths_from(&graph, "svr");
        let from_a = count_paths_from(&graph, a);
        let to_end = count_paths_to(&graph, "out");

        from_start.get(a).unwrap_or(&0) * from_a.get(b).unwrap_or(&0) * to_end.get(b).unwrap_or(&0)
    };

    count_via("fft", "dac") + count_via("dac", "fft")
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

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(read_input()), 473930047491888);
    }
}
