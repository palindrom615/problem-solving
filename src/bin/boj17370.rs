use problem_solving::utils::*;

fn solve(n: i32) -> i32 {
    0
}

fn main() {
    let n: i32 = read_line()[0];
    println!("{}", solve(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(2), 0);
        assert_eq!(solve(5), 2);
        assert_eq!(solve(8), 8);
    }
}
