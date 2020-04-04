mod utils;
use utils::*;

fn solve(n: usize, beauty2x1: &mut Vec<i64>, beauty2x2: &mut Vec<i64>) -> i64 {
    let mut answer: i64 = 0;
    beauty2x1.sort();
    beauty2x2.sort();
    if n % 2 != 0 {
        answer += beauty2x1.pop().unwrap();
    }
    let mut next_2x1_1 = beauty2x1.pop();
    let mut next_2x1_2 = beauty2x1.pop();
    let mut next_2x2 = beauty2x2.pop();
    for _ in 0..n / 2 {
        if next_2x2 == None {
            answer += next_2x1_1.unwrap() + next_2x1_2.unwrap();
            next_2x1_1 = beauty2x1.pop();
            next_2x1_2 = beauty2x1.pop();
        } else if next_2x1_2 == None
            || next_2x2.unwrap() > next_2x1_1.unwrap() + next_2x1_2.unwrap()
        {
            answer += next_2x2.unwrap();
            next_2x2 = beauty2x2.pop();
        } else {
            answer += next_2x1_1.unwrap() + next_2x1_2.unwrap();
            next_2x1_1 = beauty2x1.pop();
            next_2x1_2 = beauty2x1.pop();
        }
    }
    answer
}

fn main() {
    let n: usize = read_line()[0];
    let mut beauty2x1 = read_line();
    let mut beauty2x2 = read_line();

    println!("{}", solve(n, &mut beauty2x1, &mut beauty2x2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(5, &mut vec![1, 2, 3, 4], &mut vec![4, 5, 6]), 15);
        assert_eq!(solve(10, &mut vec![], &mut vec![5, 6, 7, 8, 9, 10]), 40);
        assert_eq!(solve(7, &mut vec![1, 10, 10], &mut vec![8, 9, 10]), 40);
        assert_eq!(solve(5, &mut vec![1, 1, 1, 1, 1], &mut vec![8]), 11);
    }
}
