use problem_solving::utils::*;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Partition {
    t: usize,
    l: usize,
    h: usize,
    w: usize,
}

fn solve(choco: Vec<Vec<usize>>) -> usize {
    let h = choco.len();
    let w = choco[0].len();
    let mut memo = HashMap::<Partition, usize>::with_capacity((h + 1) * h * (w + 1) * w);
    let ans = solve_sub(
        &choco,
        Partition {
            t: 0,
            l: 0,
            h: h,
            w: w,
        },
        &mut memo,
    );
    ans
}
fn sum_choco(choco: &Vec<Vec<usize>>, part: Partition) -> usize {
    let mut sum = 0;
    for row in part.t..part.t + part.h {
        for col in part.l..part.l + part.w {
            sum += choco[row][col];
        }
    }
    sum
}
fn solve_sub(
    choco: &Vec<Vec<usize>>,
    part: Partition,
    mut memo: &mut HashMap<Partition, usize>,
) -> usize {
    if part.w == 1 && part.h == 1 {
        0
    } else if let Some(ans) = memo.get(&part) {
        *ans
    } else {
        let mut min = std::usize::MAX;
        for p in 1..part.w {
            let subproblem = solve_sub(
                &choco,
                Partition {
                    t: part.t,
                    l: part.l,
                    h: part.h,
                    w: p,
                },
                &mut memo,
            ) + solve_sub(
                &choco,
                Partition {
                    t: part.t,
                    l: part.l + p,
                    h: part.h,
                    w: part.w - p,
                },
                &mut memo,
            );
            if min > subproblem {
                min = subproblem;
            }
        }
        for p in 1..part.h {
            let subproblem = solve_sub(
                &choco,
                Partition {
                    t: part.t,
                    l: part.l,
                    h: p,
                    w: part.w,
                },
                &mut memo,
            ) + solve_sub(
                &choco,
                Partition {
                    t: part.t + p,
                    l: part.l,
                    h: part.h - p,
                    w: part.w,
                },
                &mut memo,
            );
            if min > subproblem {
                min = subproblem;
            }
        }
        let minval = sum_choco(&choco, part) + min;
        memo.insert(part, minval);
        minval
    }
}

fn main() {
    let line = read_line();
    let n = line[0];
    // let m = line[1];
    let mut choco = Vec::with_capacity(n);
    for _ in 0..n {
        choco.push(read_line());
    }
    println!("{}", solve(choco));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(vec![vec![2, 7, 5], vec![1, 9, 5]]), 77);
    }

    #[test]
    fn test_subproblem() {
        let mut map = HashMap::new();
        assert_eq!(
            sum_choco(
                &vec![vec![1, 9, 5]],
                Partition {
                    t: 0,
                    l: 1,
                    h: 1,
                    w: 2
                }
            ),
            14
        );
        assert_eq!(
            solve_sub(
                &vec![vec![1, 9, 5]],
                Partition {
                    t: 0,
                    l: 0,
                    h: 1,
                    w: 3
                },
                &mut map
            ),
            25,
        );
        assert_eq!(
            solve_sub(
                &vec![vec![1, 9]],
                Partition {
                    t: 0,
                    l: 0,
                    h: 1,
                    w: 2
                },
                &mut map
            ),
            10,
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            sum_choco(
                &vec![vec![2, 7, 5], vec![1, 9, 5]],
                Partition {
                    l: 0,
                    t: 0,
                    w: 3,
                    h: 2
                }
            ),
            29
        );
        assert_eq!(
            sum_choco(
                &vec![vec![2, 7, 5], vec![1, 9, 5]],
                Partition {
                    l: 0,
                    t: 0,
                    w: 1,
                    h: 1
                }
            ),
            2
        );
        assert_eq!(
            sum_choco(
                &vec![vec![2, 7, 5], vec![1, 9, 5]],
                Partition {
                    l: 1,
                    t: 1,
                    w: 1,
                    h: 1
                }
            ),
            9
        );
        assert_eq!(
            sum_choco(
                &vec![vec![2, 7, 5], vec![1, 9, 5]],
                Partition {
                    l: 1,
                    t: 1,
                    w: 2,
                    h: 1
                }
            ),
            14
        );
    }
}
