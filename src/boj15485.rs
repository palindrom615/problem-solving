mod utils;
use utils::*;

/**
 * 문자열 왼쪽부터 b만 탐색하면서 b의 왼쪽에 있는 a의 개수, b의 오른쪽에 있는 c의 개수를 파악한다. 가장 먼저 나오는 b 왼쪽의
 * a의 개수를 n개, 가장 나중에 나오는 b의 오른쪽의 c의 개수를 m개, 두 b 사이의 b의 개수를 k개라고 하면 맨 왼쪽과 오른쪽의
 * b가 고정되어 있는 이 상태의 경우의 수는 (2^n - 1) * 2^k * (2^m - 1)
 *
 **/
static DIVIDER: u32 = 1_000_000_007;

fn calc_num_of(s: &String) -> (Vec<usize>, Vec<usize>) {
    let mut num_of_left_a = vec![];
    let mut num_of_right_c = vec![];
    let mut num_a = 0;
    let mut num_c = 0;
    for chr in s.chars() {
        if chr == 'a' {
            num_a += 1;
        } else if chr == 'c' {
            num_c += 1;
        } else if chr == 'b' {
            num_of_left_a.push(num_a);
            num_of_right_c.push(num_c);
        }
    }
    for i in &mut num_of_right_c {
        *i = num_c - *i;
    }
    (num_of_left_a, num_of_right_c)
}

fn solve(s: &String, two_pows: &[u32; 1_000_000]) -> u32 {
    let (num_of_left_a, num_of_right_c) = calc_num_of(s);
    let num_of_b = num_of_left_a.len();

    let mut case_of_a = Vec::with_capacity(num_of_b);
    let mut case_of_c = Vec::with_capacity(num_of_b);
    for idx in 0..num_of_b {
        case_of_a.push(two_pows[num_of_left_a[idx]] - 1);
        case_of_c.push(two_pows[num_of_right_c[idx]] - 1);
    }

    let mut case = 0;
    for i in 0..num_of_b {
        for j in i..num_of_b {
            if j - i == 0 || j - i == 1 {
                case += (case_of_a[i] * case_of_c[j]) % DIVIDER
            } else {
                case += (case_of_a[i] * two_pows[j - i - 1] * case_of_c[j]) % DIVIDER;
            }
        }
        case %= DIVIDER
    }
    case
}

fn main() {
    let mut two_pow_memo: [u32; 1_000_000] = [0; 1_000_000];
    let mut two_pow = 1;
    for i in 0..1_000_000 {
        two_pow_memo[i] = two_pow;
        two_pow = two_pow * 2 % DIVIDER;
    }

    let line = read_line::<String>();
    let s = &line[0];
    println!("{}", solve(s, &two_pow_memo));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_num_of() {
        assert_eq!(
            calc_num_of(&String::from("abccabc")),
            (vec![1, 2], vec![3, 1])
        );
    }

    #[test]
    fn examples() {
        let mut two_pow_memo: [u32; 1_000_000] = [0; 1_000_000];
        let mut two_pow = 1;
        for i in 0..1_000_000 {
            two_pow_memo[i] = two_pow;
            two_pow = two_pow * 2 % DIVIDER;
        }
        assert_eq!(solve(&"abc".to_string(), &two_pow_memo), 1);
        assert_eq!(solve(&"abcc".to_string(), &two_pow_memo), 3);
        assert_eq!(solve(&"abbcc".to_string(), &two_pow_memo), 9);
        assert_eq!(solve(&"aabbcc".to_string(), &two_pow_memo), 27);
        assert_eq!(solve(&"abbccc".to_string(), &two_pow_memo), 21);
        assert_eq!(solve(&"abbcccabc".to_string(), &two_pow_memo), 51);
        assert_eq!(solve(&"abcabcabcabc".to_string(), &two_pow_memo), 111);
        assert_eq!(solve(&"cba".to_string(), &two_pow_memo), 0);
        assert_eq!(solve(&"aaaaabbbbbccccc".to_string(), &two_pow_memo), 29791);
    }
}
