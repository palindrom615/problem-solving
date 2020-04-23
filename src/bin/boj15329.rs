use problem_solving::utils::*;

fn solve(l: i64, k: i64, memo: &mut [i64; 101]) -> i64 {
    if l <= 0 {
        return 0;
    }
    if memo[l as usize] != -1 {
        dbg!(l, memo[l as usize]);
        return memo[l as usize];
    }
    let one_thick_disc = if l >= k { 1 } else { 0 };
    let one_thin_disc = 1;
    memo[l as usize] = match l {
        0 => 0,
        1 => 1,
        _ => one_thick_disc + one_thin_disc + solve(l - 2, k, memo) + solve(l - (k + 1), k, memo),
    };
    dbg!(l, k, memo[l as usize]);
    memo[l as usize]
}

fn main() {
    let line = read_line::<i64>();
    let l = line[0];
    let k = line[1];

    let mut memo: [i64; 101] = [-1; 101];
    let answer = solve(l, k, &mut memo);

    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut memo: [i64; 101] = [-1; 101];
        assert_eq!(solve(5, 3, &mut memo), 6);
        memo = [-1; 101];
        assert_eq!(solve(9, 10, &mut memo), 5);
        memo = [-1; 101];
        assert_eq!(solve(10, 10, &mut memo), 6);
        memo = [-1; 101];
        assert_eq!(solve(20, 5, &mut memo), 86);
        memo = [-1; 101];
        assert_eq!(solve(100, 2, &mut memo), 3626169232670);
    }
}
