use std::collections::VecDeque;

/**
 * https://en.wikipedia.org/wiki/Permutation#Generation_in_lexicographic_order
 *
 * caveat: the vector must be sorted ascending
 */

struct Permutation<T: Ord + Clone> {
    xs: Vec<T>,
    n: usize,
    is_idx_used: Vec<bool>,
    digit_iter: Vec<usize>,
}

impl<T: Ord + Clone> Permutation<T> {
    fn new(xs: Vec<T>, n: usize) -> Self {
        let l = xs.len();
        Permutation {
            xs,
            n,
            is_idx_used: vec![false; l],
            digit_iter: vec![0; l],
        }
    }
    fn backtrack(&mut self, digit: usize) -> VecDeque<T> {
        unimplemented!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}
}
