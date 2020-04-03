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
