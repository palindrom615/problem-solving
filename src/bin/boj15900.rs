use problem_solving::utils::*;

use std::collections::{HashSet, VecDeque};

fn solve(num_of_nodes: usize, edges: &mut VecDeque<(usize, usize)>) -> bool {
    let mut nodes = vec![-1; num_of_nodes + 1];
    let mut leafes = HashSet::new();
    nodes[1] = 0;
    while edges.len() != 0 {
        let (n1, n2) = edges.pop_front().unwrap();
        if nodes[n1] != -1 {
            nodes[n2] = nodes[n1] + 1;
            leafes.remove(&n1);
            leafes.insert(n2);
        } else if nodes[n2] != -1 {
            nodes[n1] = nodes[n2] + 1;
            leafes.remove(&n2);
            leafes.insert(n1);
        } else {
            edges.push_back((n1, n2));
        }
    }
    let ans = leafes.iter().fold(0, |acc, &x| acc + nodes[x]);
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
    fn test_vecdeque() {
        let mut s= VecDeque::new();
        s.push_back(1);
        s.push_back(2);
        assert_eq!(s.pop_front().unwrap(), 1);
    }
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
