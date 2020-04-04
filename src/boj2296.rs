mod utils;
use utils::*;

#[derive(Copy, Clone, Debug)]
struct Building {
    x: i32,
    y: i32,
    c: i32,
}

fn solve(buildings: Vec<Building>) -> i32 {
    let mut bull_maxes = [-1; 1000];
    let mut bear_maxes = [-1; 1000];
    for (idx, building) in buildings.iter().enumerate() {
        bull_maxes[idx] = building.c;
        bear_maxes[idx] = building.c;
        for t in 0..idx {
            if buildings[t].y < building.y && bull_maxes[t] + building.c > bull_maxes[idx] {
                bull_maxes[idx] = bull_maxes[t] + building.c;
                continue;
            }
            if buildings[t].y > building.y && bear_maxes[t] + building.c > bear_maxes[idx] {
                bear_maxes[idx] = bear_maxes[t] + building.c;
                continue;
            }
        }
        dbg!(idx, bull_maxes[idx], bear_maxes[idx]);
    }
    let bull_max = *bull_maxes.iter().max().unwrap();
    let bear_max = *bear_maxes.iter().max().unwrap();
    if bull_max > bear_max {
        bull_max
    } else {
        bear_max
    }
}

fn main() {
    let t: i32 = read_line()[0];
    let mut buildings: Vec<Building> = Vec::new();
    for _ in 0..t {
        match read_line::<i32>().as_slice() {
            [x, y, c] => buildings.push(Building {
                x: *x,
                y: *y,
                c: *c,
            }),
            _ => panic!(),
        }
    }
    buildings.sort_by(|a, b| a.x.cmp(&b.x));

    println!("{}", solve(buildings));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let buildings = vec![
            Building { x: 1, y: 1, c: 2 },
            Building { x: 2, y: 5, c: 4 },
            Building { x: 4, y: 6, c: 2 },
            Building { x: 5, y: 2, c: 5 },
        ];
        assert_eq!(solve(buildings), 9);

        let mut buildings = vec![
            Building { x: 1, y: 1, c: 2 },
            Building { x: 2, y: 5, c: 4 },
            Building { x: 4, y: 6, c: 2 },
            Building { x: 5, y: 2, c: 5 },
            Building { x: 6, y: 3, c: 5 },
        ];
        buildings.sort_by(|a, b| a.x.cmp(&b.x));
        assert_eq!(solve(buildings), 12);

        let mut buildings = vec![
            Building { x: 1, y: 1, c: 2 },
            Building { x: 2, y: 5, c: 4 },
            Building { x: 4, y: 6, c: 2 },
            Building { x: 5, y: 2, c: 5 },
            Building { x: 6, y: 3, c: 5 },
            Building {
                x: 100,
                y: 100,
                c: 100,
            },
        ];
        buildings.sort_by(|a, b| a.x.cmp(&b.x));
        assert_eq!(solve(buildings), 112);
    }
}
