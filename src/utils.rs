use std::str::FromStr;
use std::io::stdin;

pub fn read_line<T: FromStr>() -> Vec<T> {
    let mut line = String::new();
    let _ = stdin().read_line(&mut line);
    line.split_whitespace()
        .flat_map(|x| x.parse::<T>())
        .collect()
}

pub mod modular;
pub mod combination;
pub mod permutation;
