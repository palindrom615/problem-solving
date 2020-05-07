use problem_solving::utils::*;

use std::collections::HashSet;
use std::iter::FromIterator;

const MASK_TRAIN: u32 = 0b0000_0000_0000_1111_1111_1111_1111_1111;

fn main() {
    let line: Vec<usize> = read_line();
    let n = line[0];
    let m = line[1];
    let mut trains: Vec<u32> = vec![0; n];
    for _ in 0..m {
        match read_line::<usize, Vec<usize>>().as_slice() {
            [1, i, x] => trains[*i - 1] |= (1 << *x - 1) as u32,
            [2, i, x] => trains[*i - 1] &= !(1 << *x - 1) as u32,
            [3, i] => trains[*i - 1] = (trains[*i - 1] << 1) & MASK_TRAIN,
            [4, i] => trains[*i - 1] >>= 1,
            _ => (),
        }
    }
    let trains_galaxy = HashSet::<u32>::from_iter(trains);
    println!("{}", trains_galaxy.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}
}
