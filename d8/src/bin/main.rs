use d8::solve_1;
use d8::solve_2;

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input");

    let solution1 = solve_1(input, 1000);
    println!("Solution 1: {}", solution1);

    let solution2 = solve_2(input);
    println!("Solution 2: {}", solution2);

    divan::main();
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_1() {
        let input = include_str!("../../test");

        assert_eq!(solve_1(input, 10), 40);
    }

    #[test]
    fn test_solve_2() {
        let input = include_str!("../../test");

        assert_eq!(solve_2(input), 25272);
    }
}
