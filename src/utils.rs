use std::str::FromStr;
use std::io::stdin;
use std::iter::FromIterator;

pub fn read_line<T: FromStr, B: FromIterator<T>>() -> B {
    let mut line = String::new();
    let _ = stdin().read_line(&mut line);
    line.split_whitespace()
        .flat_map(|x| x.parse::<T>())
        .collect()
}

pub mod modular;
pub mod combination;
pub mod permutation;
