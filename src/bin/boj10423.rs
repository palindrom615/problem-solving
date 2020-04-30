use problem_solving::utils::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Eq, Clone, Copy)]
struct Edge {
    cities: (usize, usize),
    value: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.cmp(&self.value)
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(n: usize, connected: &mut HashSet<usize>, mut edges: Vec<Edge>) -> usize {
    let mut res = 0;
    let is_rim = |edge: &Edge, connected: &HashSet<usize>| {
        let (u, v) = edge.cities;
        let u_connected = connected.contains(&u);
        let v_connected = connected.contains(&v);
        v_connected || u_connected
    };
    let mut possible_edges: BinaryHeap<Edge> = edges
        .iter()
        .filter(|&edge| is_rim(edge, &connected))
        .map(|edge| *edge)
        .collect();
    while let Some(edge) = possible_edges.pop() {
        if connected.len() == n {
            break;
        }

        let (u, v) = edge.cities;
        let u_connected = connected.contains(&u);
        let v_connected = connected.contains(&v);
        if u_connected && v_connected {
            continue;
        }

        res += edge.value;
        connected.insert(v);
        connected.insert(u);

        let (rim, not_rim) = edges.iter().partition(|&edge| is_rim(edge, &connected));
        possible_edges.extend(rim);
        edges = not_rim;
    }
    res
}

fn main() {
    let line: Vec<usize> = read_line();
    let n = line[0];
    let m = line[1];
    let mut connected: HashSet<usize> = read_line();
    let mut edges: Vec<Edge> = Vec::with_capacity(m);
    for _ in 0..m {
        let line: Vec<usize> = read_line();
        edges.push(Edge {
            cities: (line[0], line[1]),
            value: line[2],
        });
    }
    println!("{}", solve(n, &mut connected, edges));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn examples() {
        let mut connected = HashSet::from_iter(vec![1, 2, 9]);
        let mut edges = vec![
            Edge {
                cities: (3, 4),
                value: 11,
            },
            Edge {
                cities: (2, 4),
                value: 10,
            },
            Edge {
                cities: (4, 6),
                value: 10,
            },
            Edge {
                cities: (5, 6),
                value: 5,
            },
            Edge {
                cities: (6, 7),
                value: 7,
            },
            Edge {
                cities: (7, 8),
                value: 5,
            },
            Edge {
                cities: (1, 4),
                value: 8,
            },
            Edge {
                cities: (1, 3),
                value: 3,
            },
            Edge {
                cities: (5, 7),
                value: 4,
            },
            Edge {
                cities: (3, 5),
                value: 6,
            },
            Edge {
                cities: (6, 8),
                value: 4,
            },
            Edge {
                cities: (4, 5),
                value: 4,
            },
            Edge {
                cities: (7, 9),
                value: 2,
            },
            Edge {
                cities: (8, 9),
                value: 5,
            },
        ];
        assert_eq!(solve(9, &mut connected, edges), 22);

        let mut connected = HashSet::from_iter(vec![1]);
        let mut edges = vec![
            Edge {
                cities: (1, 2),
                value: 5,
            },
            Edge {
                cities: (1, 3),
                value: 5,
            },
            Edge {
                cities: (1, 4),
                value: 5,
            },
            Edge {
                cities: (2, 3),
                value: 10,
            },
            Edge {
                cities: (3, 4),
                value: 10,
            },
        ];
        assert_eq!(solve(4, &mut connected, edges), 15);

        let mut connected = HashSet::from_iter(vec![4, 6, 1, 9, 10]);
        let mut edges = vec![
            Edge {
                cities: (4, 5),
                value: 1,
            },
            Edge {
                cities: (9, 10),
                value: 1,
            },
            Edge {
                cities: (7, 8),
                value: 3,
            },
            Edge {
                cities: (5, 6),
                value: 2,
            },
            Edge {
                cities: (1, 2),
                value: 3,
            },
            Edge {
                cities: (6, 7),
                value: 6,
            },
            Edge {
                cities: (3, 4),
                value: 5,
            },
            Edge {
                cities: (2, 3),
                value: 8,
            },
            Edge {
                cities: (8, 9),
                value: 4,
            },
        ];
        assert_eq!(solve(10, &mut connected, edges), 16);
    }

    #[test]
    fn examples_complement() {
        let mut edges = vec![
            Edge {
                cities: (2, 3),
                value: 1,
            },
            Edge {
                cities: (3, 4),
                value: 3,
            },
            Edge {
                cities: (2, 4),
                value: 2,
            },
            Edge {
                cities: (1, 4),
                value: 10,
            },
        ];
        let mut connected = HashSet::from_iter(vec![1]);
        assert_eq!(solve(4, &mut connected, edges), 13);
    }
}
