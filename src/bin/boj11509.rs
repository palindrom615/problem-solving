use problem_solving::utils::*;

fn solve(balloons: Vec<usize>) -> usize {
    let mut arrows = Vec::new();
    for balloon in balloons {
        if let Some(arrow) = arrows.iter().position(|&arrow| arrow == balloon) {
            arrows[arrow] -= 1;
        } else {
            arrows.push(balloon - 1);
        }
    }
    arrows.len()
}

fn main() {
    let n: usize = read_line()[0];
    let mut balloons = read_line();
    println!("{}", solve(balloons));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(vec![2, 1, 5, 4, 3]), 2);
        assert_eq!(solve(vec![1, 2, 3, 4, 5]), 5);
        assert_eq!(solve(vec![4, 5, 2, 1, 4]), 3);
    }

    #[test]
    fn examples_complement() {
        assert_eq!(solve((1..1001).collect()), 1000);
        assert_eq!(solve((1..1001).rev().collect()), 1);
        assert_eq!(solve(vec![4, 5, 4, 3, 3]), 2);
    }
}
