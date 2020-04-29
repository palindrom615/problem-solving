use problem_solving::utils::*;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    idx: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x + self.y).cmp(&(other.x + other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let n: usize = read_line()[0];
    let mut points = BinaryHeap::new();
    for idx in 1..3 * n + 1 {
        let line = read_line();
        points.push(Point {
            x: line[0],
            y: line[1],
            idx,
        });
    }
    // dbg!(&points);
    while !points.is_empty() {
        let p1 = points.pop().unwrap().idx;
        let p2 = points.pop().unwrap().idx;
        let p3 = points.pop().unwrap().idx;
        println!("{} {} {}", p1, p2, p3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}
}
