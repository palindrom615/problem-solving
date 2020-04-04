mod utils;
use utils::*;

fn solve(p: i64, times: Vec<i64>) -> (i64, i64) {
    let mut num = 0;
    let mut penalty = 0;
    let mut time = p - 1;
    for t in times.iter() {
        if time < *t {
            break;
        }
        num += 1;
        penalty += time;
        time -= t;
    }
    (num, penalty)
}
fn main() {
    let line: Vec<i64> = read_line();
    let p = line[0];

    let mut times: Vec<i64> = read_line();
    times.sort();
    let (num, penalty) = solve(p, times);
    println!("{} {}", num, penalty);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(30, vec![2, 12, 16]), (2, 56));
        assert_eq!(solve(40, vec![2, 12, 16]), (3, 101));
    }
}
