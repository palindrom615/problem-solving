mod data_structures;
mod utils;
use data_structures::smmh::*;
use std::collections::HashSet;
use utils::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Composite {
    value: i32,
    powers: Vec<u32>,
}

fn solve<'a>(k: usize, n: usize, primes: Vec<i32>) -> i32 {
    let mut heap = SMMH::with_capacity(n);
    let mut set = HashSet::new();

    heap.push(Composite {
        value: 1,
        powers: vec![0; primes.len()],
    });
    for i in 0..n {
        if let Some(min) = heap.pop_min() {
            set.remove(&format!("{:?}", &min.powers));
            for idx in 0..k {
                let mut power = min.powers.clone();
                power[idx] += 1;
                if set.contains(&format!("{:?}", power)) {
                    continue;
                }
                let new_val = min.value * primes[idx];
                if heap.len() > n - i + 1 {
                    let max_val = heap.peek_max().unwrap();
                    if new_val > max_val.value {
                        continue;
                    }
                }
                set.insert(format!("{:?}", &power));
                heap.push(Composite {
                    value: new_val,
                    powers: power,
                });

                if heap.len() > n - i + 1{
                    let rm = heap.pop_max().unwrap();
                    set.remove(&format!("{:?}", rm.powers));
                }
            }
        }
    }
    let min = heap.pop_min().unwrap();
    min.value
}

fn main() {
    let line: Vec<usize> = read_line();
    let k = line[0];
    let n = line[1];
    let primes: Vec<i32> = read_line();

    println!("{}", solve(k, n, primes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(4, 19, vec![2, 3, 5, 7]), 27);
    }
    #[test]
    fn memory() {
        assert_eq!(
            solve(
                78,
                100000,
                vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
                    163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241,
                    251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347,
                    349, 353, 359, 367, 373, 379, 383, 389, 397
                ]
            ),
            343170
        );
    }
}
