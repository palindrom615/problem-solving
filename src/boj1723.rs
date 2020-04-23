mod utils;
use utils::*;

fn solve(angles: Vec<f64>, k: usize) -> usize {
// 가장 많은 차이가 나도록 하는 분배를 찾기 -> 가장 많은 조각을 반으로 나누기?
    let angle_of_sector: f64 = 360. / k as f64;
    let mut max = 0;
    for (i, &a) in angles.iter().enumerate() {
        let pop = angles[i..].iter()
            .take_while(|&x| *x < a + angle_of_sector).count();
        if pop > max {
            max = pop;
        }
    }
    1
}

fn main() {
    let line = read_line();
    let n = line[0];
    let k = line[1];
    let mut angles: Vec<f64> = Vec::with_capacity(n);
    for i in 0..n {
        angles.push(read_line()[0]);
    }
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{}", solve(angles, k));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solve(vec![30., 60., 150.003, 240.], 4), 1)
    }
}
