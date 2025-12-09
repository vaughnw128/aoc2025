fn main() {
    let n = 100;

    let mut sum_sq: u32 = 0;
    let mut sq_sum: u32 = 0;

    for i in 1_u32..=n {
        sum_sq = sum_sq + i.pow(2);
        sq_sum += i;
    }
    sq_sum = sq_sum.pow(2);

    dbg!(sq_sum - sum_sq);
}
