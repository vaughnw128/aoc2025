use d5::solve_1;
use d5::solve_2;

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let input = include_str!("../../input");

    let solution1 = solve_1(input);
    println!("Solution 1: {}", solution1);

    let solution2 = solve_2(input);
    println!("Solution 2: {}", solution2);

    divan::main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve_1() {
            let input = include_str!("../../test");

            assert_eq!(solve_1(input), 3);
        }

        #[test]
        fn test_solve_2() {
            let input = include_str!("../../test");

            assert_eq!(solve_2(input), 14);
        }
    }
}

