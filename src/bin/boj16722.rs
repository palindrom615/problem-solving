use problem_solving::utils::*;
use problem_solving::utils::combination::*;

use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    C,
    S,
    T,
}
impl Shape {
    fn from(shape: &str) -> Shape {
        match shape {
            "CIRCLE" => Shape::C,
            "SQUARE" => Shape::S,
            "TRIANGLE" => Shape::T,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Y,
    R,
    B,
}
impl Color {
    fn from(color: &str) -> Color {
        match color {
            "YELLOW" => Color::Y,
            "RED" => Color::R,
            "BLUE" => Color::B,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BgColor {
    G,
    K,
    W,
}
impl BgColor {
    fn from(color: &str) -> BgColor {
        match color {
            "GRAY" => BgColor::G,
            "BLACK" => BgColor::K,
            "WHITE" => BgColor::W,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pic {
    idx: i32,
    shape: Shape,
    color: Color,
    bg_color: BgColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Combination<'a> {
    pics: [&'a Pic; 3],
}
impl Combination<'_> {
    fn is_h(&self) -> bool {
        let is_shape_h = is_h(self.pics[0].shape, self.pics[1].shape, self.pics[2].shape);
        let is_color_h = is_h(self.pics[0].color, self.pics[1].color, self.pics[2].color);
        let is_bg_color_h = is_h(
            self.pics[0].bg_color,
            self.pics[1].bg_color,
            self.pics[2].bg_color,
        );

        is_shape_h && is_color_h && is_bg_color_h
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Guess {
    H([i32; 3]),
    G,
}
impl Guess {
    fn from(ans: Vec<String>) -> Guess {
        match &ans[0][..] {
            "H" => {
                let p1 = ans[1].parse::<i32>().unwrap();
                let p2 = ans[2].parse::<i32>().unwrap();
                let p3 = ans[3].parse::<i32>().unwrap();
                let mut p = [p1, p2, p3];
                p.sort();
                Guess::H(p)
            }
            "G" => Guess::G,
            _ => panic!(),
        }
    }
}
fn calc_score(guesses: Vec<Guess>, all_h_set: &HashSet<[i32; 3]>) -> i32 {
    let mut score = 0;
    let mut answered_pics = HashSet::with_capacity(9);
    let mut right_hs = 0;
    let mut right_gs = 0;
    for g in guesses {
        if let Guess::H(p) = g {
            if all_h_set.contains(&p) && !answered_pics.contains(&p) {
                answered_pics.insert(p);
                score += 1;
                right_hs += 1;
            } else {
                score -= 1;
            }
        } else {
            if right_gs == 0 && right_hs == all_h_set.len() {
                score += 3;
                right_gs += 1;
            } else {
                score -= 1;
            }
        }
    }
    score
}

fn main() {
    let mut pics = Vec::with_capacity(9);
    for i in 1..10 {
        let line: Vec<String> = read_line();
        pics.push(Pic {
            idx: i,
            shape: Shape::from(&line[0]),
            color: Color::from(&line[1]),
            bg_color: BgColor::from(&line[2]),
        });
    }

    let all_comb = combination(3, pics);
    let all_h: Vec<Combination> = all_comb
        .iter()
        .map(|a| Combination {
            pics: [&a[0], &a[1], &a[2]],
        })
        .filter(|&a| a.is_h())
        .collect();
    let mut all_h_set = HashSet::with_capacity(9);
    for h in all_h.iter() {
        all_h_set.insert([h.pics[0].idx, h.pics[1].idx, h.pics[2].idx]);
    }

    let n = read_line()[0];

    let mut guesses = Vec::new();
    for _ in 0..n {
        guesses.push(Guess::from(read_line()));
    }
    println!("{}", calc_score(guesses, &all_h_set));
}

fn are_three_same<T: Eq>(a: T, b: T, c: T) -> bool {
    return a == b && b == c;
}

fn are_three_different<T: Eq>(a: T, b: T, c: T) -> bool {
    return a != b && b != c && c != a;
}

fn is_h<T: Eq + Copy>(a: T, b: T, c: T) -> bool {
    are_three_same(a, b, c) || are_three_different(a, b, c)
}

#[cfg(test)]
mod tests {
    use super::BgColor::*;
    use super::Color::*;
    use super::Shape::*;
    use super::*;

    #[test]
    fn examples() {
        let pics = vec![
            Pic {
                idx: 1,
                shape: C,
                color: Y,
                bg_color: G,
            },
            Pic {
                idx: 2,
                shape: C,
                color: R,
                bg_color: K,
            },
            Pic {
                idx: 3,
                shape: C,
                color: R,
                bg_color: G,
            },
            Pic {
                idx: 4,
                shape: C,
                color: Y,
                bg_color: K,
            },
            Pic {
                idx: 5,
                shape: C,
                color: R,
                bg_color: W,
            },
            Pic {
                idx: 6,
                shape: C,
                color: B,
                bg_color: K,
            },
            Pic {
                idx: 7,
                shape: S,
                color: Y,
                bg_color: G,
            },
            Pic {
                idx: 8,
                shape: S,
                color: B,
                bg_color: G,
            },
            Pic {
                idx: 9,
                shape: T,
                color: B,
                bg_color: W,
            },
        ];
        let all_comb = combination(3, pics);
        let all_h: Vec<Combination> = all_comb
            .iter()
            .map(|a| Combination {
                pics: [&a[0], &a[1], &a[2]],
            })
            .filter(|&a| a.is_h())
            .collect();
        let mut all_h_set = HashSet::with_capacity(9);
        for h in all_h.iter() {
            all_h_set.insert([h.pics[0].idx, h.pics[1].idx, h.pics[2].idx]);
        }
        let guesses = vec![
            "H 1 6 5", "H 7 8 9", "H 2 3 5", "H 1 5 6", "H 6 8 9", "G", "H 2 4 6", "H 9 7 2", "G",
        ]
        .iter()
        .map(|l| l.split_whitespace().map(|x| String::from(x)).collect())
        .map(|g| Guess::from(g))
        .collect();
        assert_eq!(calc_score(guesses, &all_h_set), 5);
        let guesses = vec![
            "H 1 6 5", "H 7 8 9", "H 2 3 5", "H 1 5 6", "H 6 8 9", "G", "H 2 4 6", "H 9 7 2", "G",
            "G",
        ]
        .iter()
        .map(|l| l.split_whitespace().map(|x| String::from(x)).collect())
        .map(|g| Guess::from(g))
        .collect();
        assert_eq!(calc_score(guesses, &all_h_set), 4);
    }
    #[test]
    fn is_h() {
        let pics = vec![
            Pic {
                idx: 1,
                shape: C,
                color: Y,
                bg_color: G,
            },
            Pic {
                idx: 2,
                shape: C,
                color: R,
                bg_color: K,
            },
            Pic {
                idx: 3,
                shape: C,
                color: R,
                bg_color: G,
            },
            Pic {
                idx: 4,
                shape: C,
                color: Y,
                bg_color: K,
            },
            Pic {
                idx: 5,
                shape: C,
                color: R,
                bg_color: W,
            },
            Pic {
                idx: 6,
                shape: C,
                color: B,
                bg_color: K,
            },
            Pic {
                idx: 7,
                shape: S,
                color: Y,
                bg_color: G,
            },
            Pic {
                idx: 8,
                shape: S,
                color: B,
                bg_color: G,
            },
            Pic {
                idx: 9,
                shape: T,
                color: B,
                bg_color: W,
            },
        ];
        let com1 = Combination {
            pics: [&pics[0], &pics[4], &pics[5]],
        };
        let com2 = Combination {
            pics: [&pics[0], &pics[1], &pics[2]],
        };
        assert!(com1.is_h());
        assert!(!com2.is_h());
    }
}
