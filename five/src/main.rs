// https://adventofcode.com/2025/day/5

use std::fs::read_to_string;
use std::cmp::{min, max};

fn if_overlap(r1: &(u64, u64), r2: &(u64, u64)) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}

fn main() {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    let mut total = 0;
    for line in read_to_string("input").unwrap().lines() {
        if line == "" { break; }

        // transform into vec of tuples so we can easily sort and parse out the ranges
        let nums = line
            .split('-')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        ranges.push((nums[0], nums[1]));
    }

    // union any overlapped ranges
    for i in 0..ranges.len() {
        for j in i+1..ranges.len() {
            if if_overlap(&ranges[i], &ranges[j]) {
                let new_range = (
                    min(ranges[i].0, ranges[j].0),
                    max(ranges[i].1, ranges[j].1),
                );
                ranges[j] = new_range;
                ranges[i] = (0, 0); // mark for removal so we don't edit in place
            }}
        }

    // count remaining ranges ignoring culled ones
    for r in ranges {
        if r.0 == 0 { continue; }
        total += r.1 - r.0 + 1;
    }

    println!("Sum: {}", total);
}
