use problem_solving::utils::*;

fn fib(n: usize, memo: &mut [u64; 81]) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    }
    if memo[n] != 0 {
        return memo[n];
    }

    memo[n] = fib(n - 1, memo) + fib(n - 2, memo);
    return memo[n];
}

fn solve(n: usize) -> u64 {
    let mut fib_memo = [0; 81];
    2 * (2 * fib(n, &mut fib_memo) + fib(n - 1, &mut fib_memo))
}

fn main() {
    let n = read_line()[0];
    println!("{}", solve(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(1), 4);
        assert_eq!(solve(2), 6);

        assert_eq!(solve(5), 26);
        assert_eq!(solve(6), 42);
        assert_eq!(solve(80), 122_611_581_443_223_182);

    }
}
