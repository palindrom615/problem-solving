mod utils;
use utils::*;

fn solve(cards: Vec<usize>, mut memo: &mut Vec<Vec<(usize, bool)>>) -> usize {
    sub_solve(&cards, 0, cards.len() - 1, &mut memo).0
}

fn sub_solve(
    cards: &Vec<usize>,
    head: usize,
    tail: usize,
    mut memo: &mut Vec<Vec<(usize, bool)>>,
) -> (usize, bool) {
    if tail - head == 0 {
        (cards[head], true)
    } else if tail - head == 1 {
        if cards[head] > cards[tail] {
            (cards[head], true)
        } else {
            (cards[tail], false)
        }
    } else if memo[head][tail].0 != 0 {
        memo[head][tail]
    } else {
        let (head_opponent_best, head_opponent_pick_is_head) =
            sub_solve(cards, head + 1, tail, &mut memo);
        let head_max_score = cards[head]
            + if head_opponent_pick_is_head {
                sub_solve(cards, head + 2, tail, &mut memo).0
            } else {
                sub_solve(cards, head + 1, tail - 1, &mut memo).0
            };
        let (tail_opponent_best, tail_opponent_pick_is_head) =
            sub_solve(cards, head, tail - 1, &mut memo);
        let tail_max_score = cards[tail]
            + if tail_opponent_pick_is_head {
                sub_solve(cards, head + 1, tail - 1, &mut memo).0
            } else {
                sub_solve(cards, head, tail - 2, &mut memo).0
            };
        if head_max_score > tail_max_score {
            memo[head][tail] = (head_max_score, true);
            memo[head][tail]
        } else {
            memo[head][tail] = (tail_max_score, false);
            memo[head][tail]
        }
    }
}
fn main() {
    let t: i32 = read_line()[0];
    for i in 0..t {
        let _ = read_line::<usize>();
        let cards = read_line();
        let mut memo = vec![vec![(0, true); 1000]; 1000];

        println!("{}", solve(cards, &mut memo));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let mut memo = vec![vec![(0, true); 1000]; 1000];

        assert_eq!(solve(vec![1, 2, 5, 2], &mut memo), 6);
        let mut memo = vec![vec![(0, true); 1000]; 1000];

        assert_eq!(solve(vec![1, 1, 1, 1, 2, 2, 2, 2, 2], &mut memo), 8);
    }
}
