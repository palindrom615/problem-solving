mod utils;
use utils::*;

use std::collections::BTreeMap;

fn main() {
    let line: Vec<usize> = read_line();
    let n = line[0];
    let q = line[1];

    let mut flow_sum = BTreeMap::new();
    for _ in 0..q {
        let line: Vec<i32> = read_line();
        if line[0] == 1 {
            let (_, &before) = flow_sum.range(..=line[1]).next_back().unwrap_or((&0, &0));
            flow_sum.insert(line[1], before + line[2]);
            for (_, sum) in flow_sum.range_mut(line[1] + 1..) {
                *sum += line[2];
            }
        } else {
            let (_, &before) = flow_sum.range(..line[1]).next_back().unwrap_or((&0, &0));
            let (_, &after) = flow_sum
                .range(..=line[2])
                .next_back()
                .unwrap_or(flow_sum.iter().next_back().unwrap_or((&0, &0)));
            println!("{}", after - before);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}
}
