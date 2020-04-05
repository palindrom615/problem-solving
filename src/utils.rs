use std::fmt::Debug;
use std::str::FromStr;

pub fn read_line<T: FromStr>() -> Vec<T>
where
    T::Err: Debug,
{
    let stdin = std::io::stdin();
    let mut line = String::new();
    let _ = stdin.read_line(&mut line);
    line.split_whitespace()
        .map(|x| String::from(x).parse::<T>().unwrap())
        .collect()
}

pub fn combination<T: Copy>(n: usize, xs: Vec<T>) -> Vec<Vec<T>> {
    let mut pointers: Vec<usize> = (0..n).collect();
    let mut res: Vec<Vec<T>> = Vec::new();
    res.push(pointers.iter().map(|&a| xs[a]).collect());
    while pointers[0] != xs.len() - n {
        for idx in (0..n).rev() {
            pointers[idx] += 1;
            for i in idx + 1..n {
                pointers[i] = pointers[idx] + i - idx;
            }
            res.push(pointers.iter().map(|&a| xs[a]).collect());
            if pointers[idx] != xs.len() - n + idx {
                break;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_combintation() {
        assert_eq!(
            combination(2, vec![1, 2, 3]),
            vec![vec![1, 2], vec![1, 3], vec![2, 3]]
        );
    }
}
