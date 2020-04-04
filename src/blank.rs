mod utils;
use utils::*;

fn main() {
    let t: i32 = read_line()[0];
    for i in 0..t {
        let answer = 0;
        /////////////////////////////////////////////////////////////////////////////////////////////
        /*
          Implement your algorithm here.
          The answer to the case will be stored in variable answer.
        */
        /////////////////////////////////////////////////////////////////////////////////////////////

        println!("Case #{}", i + 1);
        println!("{}", answer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {}
}
