mod utils;
use utils::*;

use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
struct Node {
    depth: i32,
}

fn solve(num_of_nodes: usize, edges: &mut VecDeque<(usize, usize)>) -> bool {
    let mut nodes = vec![Node { depth: -1 }; num_of_nodes + 1];
    let mut leafes: HashSet<usize> = (1..num_of_nodes + 1).collect();
    nodes[1].depth = 0;
    while edges.len() != 0 {
        let (n1, n2) = edges.pop_front().unwrap();
        if nodes[n1].depth != -1 {
            nodes[n2].depth = nodes[n1].depth + 1;
            leafes.remove(&n1);
        } else if nodes[n2].depth != -1 {
            nodes[n1].depth = nodes[n2].depth + 1;
            leafes.remove(&n2);
        } else {
            edges.push_back((n1, n2));
        }
    }
    let ans = leafes.iter().fold(0, |acc, &x| acc + nodes[x].depth);
    ans % 2 == 1
}

fn main() {
    let t: usize = read_line()[0];
    let mut edge_queue = VecDeque::with_capacity(t - 1);
    for _ in 0..t - 1 {
        let line = read_line();
        edge_queue.push_back((line[0], line[1]));
    }
    let ans = solve(t, &mut edge_queue);
    println!("{}", if ans { "Yes" } else { "No" });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(2, &mut vec![(2, 1)].into()), true);
        assert_eq!(solve(4, &mut vec![(1, 2), (2, 3), (2, 4)].into()), false);
        assert_eq!(
            solve(
                8,
                &mut vec![(8, 1), (1, 4), (7, 4), (6, 4), (6, 5), (1, 3), (2, 3)].into()
            ),
            false
        );
    }
}
