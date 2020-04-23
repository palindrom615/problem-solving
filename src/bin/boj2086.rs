use problem_solving::utils::*;

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 || n == 2 {
        0
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
fn sum_fib(n: usize) -> usize {
    fib(n + 2) - 1
}

fn main() {
    let line = read_line();
    let a: usize = line[0];
    let b = line[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}
}
