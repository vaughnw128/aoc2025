fn main() {
    let n: u64 = 13195;

    let mut factors = Vec::new();
    let count = 1;
    while n > 1 {
        for i in 2..n {
            if n % i == 0 {
                factors.push(i);
                break;
            }
        }
        dbg!(&factors);
    }
}