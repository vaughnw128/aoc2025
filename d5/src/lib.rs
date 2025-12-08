use std::cmp::{min, max};

fn if_overlap(r1: &(u64, u64), r2: &(u64, u64)) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}

fn process(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients = Vec::new();
    let mut ranges_time = true;
    for line in input.lines() {
        if line.is_empty() { ranges_time = false; continue; }
        if ranges_time {
            let nums = line
                .split('-')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            ranges.push((nums[0], nums[1]));
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }

    }

    for i in 0..ranges.len() {
        for j in i + 1..ranges.len() {
            if if_overlap(&ranges[i], &ranges[j]) {
                let new_range = (
                    min(ranges[i].0, ranges[j].0),
                    max(ranges[i].1, ranges[j].1),
                );
                ranges[j] = new_range;
                ranges[i] = (0, 0);
            }
        }
    }

    ranges = ranges.into_iter().filter(|r| r.0 != 0).collect();

    (ranges, ingredients)
}

pub fn solve_1(input: &str) -> u64 {
    let (ranges, ingredients) = process(input);
    let mut count = 0;
    for i in ingredients {
        for r in ranges.iter() {
            if i >= r.0 && i <= r.1 { count += 1; }
        }
    }
    count
}

pub fn solve_2(input: &str) -> u64 {
    let (ranges, _) = process(input);
    let mut count = 0;
    for r in ranges {
        count += r.1 - r.0 + 1;
    }

    count
}

