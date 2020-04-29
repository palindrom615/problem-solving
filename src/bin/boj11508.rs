use problem_solving::utils::*;

fn solve(mut c: Vec<usize>) -> usize {
    c.sort_by(|x, y| y.cmp(x));
    (0..c.len())
        .filter(|x| x % 3 != 2)
        .fold(0, |acc, x| acc + c[x])
}
fn main() {
    let n = read_line()[0];
    let mut c: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        c.push(read_line()[0]);
    }
    println!("{}", solve(c));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(vec![3, 2, 3, 2]), 8);
        assert_eq!(solve(vec![6, 4, 5, 5, 5, 5]), 21);
    }
}
