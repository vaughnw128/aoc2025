fn main() {
    dbg!(
        &[1; 1000]
            .to_vec()
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 3 == 0 || i % 5 == 0)
            .map(|(i, _)| i)
            .sum::<usize>()
    );
}
