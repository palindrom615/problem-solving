mod data_structures;
mod utils;
use data_structures::smmh::*;
use std::collections::HashSet;
use utils::*;

fn solve<'a>(n: usize, primes: Vec<i32>) -> i32 {
    let mut heap = SMMH::with_capacity(n);
    let mut set = HashSet::with_capacity(n);

    heap.push(1);
    for i in 0..n {
        if let Some(min) = heap.pop_min() {
            set.remove(&min);
            let &max_val = heap.peek_max().unwrap_or(&i32::max_value());

            println!("{}, {} {}", min, heap.len(), max_val);
            for p in primes.iter() {
                let new_val = min * p;
                if set.contains(&new_val) {
                    continue;
                }
                let &max_val = heap.peek_max().unwrap_or(&i32::max_value());
                if heap.len() > n  {
                    if new_val > max_val {
                        break;
                    } else {
                        set.remove(&heap.pop_max().unwrap());
                    }
                }
                set.insert(new_val);
                heap.push(new_val);
            }
        }
    }
    heap.pop_min().unwrap()
}

fn main() {
    let line: Vec<usize> = read_line();
    // let k = line[0];
    let n = line[1];
    let primes: Vec<i32> = read_line();

    println!("{}", solve(n, primes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(19, vec![2, 3, 5, 7]), 27);
    }
    #[test]
    fn memory() {
        assert_eq!(
            solve(
                397,
                vec![
                    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
                    79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157,
                    163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241,
                    251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347,
                    349, 353, 359, 367, 373, 379, 383, 389, 397
                ]
            ),
            397
        );
    }
}
