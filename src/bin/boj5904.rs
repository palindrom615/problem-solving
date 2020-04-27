/**
 * n(k) := number of characters in S(k)
 * n(k) = 2n(k-1) + k + 3
 * n(k) + k + 5 = 2n(k - 1) + 2(k - 1) + 10
 * m(k) := n(k) + k + 5 = 2m(k - 1) = m(0) * 2^(k) = 2^(k+1)
 * n(k) = 2^(k+3) - k - 5
 */
use problem_solving::utils::*;

fn n_k(k: i32) -> usize {
    usize::pow(2, (k + 3) as u32) - (k + 5) as usize
}

fn solve(n: usize) -> String {
    let i = (-1..).find(|&x| n_k(x) < n && n <= n_k(x + 1)).unwrap();
    match n - n_k(i) {
        1 => "m".to_string(),
        j if j <= (i + 4) as usize => "o".to_string(),
        _ => solve(n - n_k(i) - (i + 4) as usize),
    }
}

fn main() {
    let n: usize = read_line()[0];
    println!("{}", solve(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moo(n: usize) -> String {
        let mid = "m".to_string() + &"o".repeat(n + 2);
        match n {
            0 => "moo".to_string(),
            n => moo(n - 1) + &mid + &moo(n - 1),
        }
    }
    #[test]
    fn test_moo() {
        assert_eq!(moo(0), "moo");
        assert_eq!(moo(1), "moomooomoo");
    }
    #[test]
    fn test_n_k() {
        assert_eq!(n_k(-1), 0);
        assert_eq!(n_k(0), 3);
        assert_eq!(n_k(1), 10);
    }

    #[test]
    fn examples() {
        let mooo = moo(10);
        let idx = 8000;
        assert_eq!(solve(idx), mooo.get((idx - 1)..idx).unwrap());
    }
}
