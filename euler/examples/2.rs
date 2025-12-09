fn main() {
    let n = 4_000_000;

    let mut fib = vec![1, 2];
    while *fib.last().unwrap() < n {
        fib.push(fib[fib.len() - 2] + fib[fib.len() - 1]);
    }

    dbg!(fib.iter().filter(|f| *f % 2 == 0).sum::<u32>());
}
