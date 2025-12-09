fn main() {
    dbg!(
        &[1; 730_000]
            .to_vec()
            .iter()
            .enumerate()
            .map(|(i, _)| (i + 1).pow(2))
            .filter(|n| n % 2 != 0)
            .sum::<usize>()
    );
}
