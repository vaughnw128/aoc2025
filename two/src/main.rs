// https://adventofcode.com/2025/day/2

use std::env;
use std::fs::read_to_string;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = &args[1];

    let input = read_to_string(input).unwrap();
    let mut total: usize = 0;

    // big ass array of ids
    for (start, end) in input
        .split(',')
        .map(|s| {
            let nums = s
                .split('-')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (nums[0], nums[1])
        })
    {
        for id in start..=end {
            // just some simpler shorthand
            let id_str = id.to_string();
            let l = id_str.len();
            let max_len = if l > 2 {
                l / 2
            } else {
                l
            };

            // we can basically just split everything up into substrings from 1 to either length or l / 2
            // and check if every single one of them is the same
            let mut chunk_invalid = 0;
            for i in 1..=max_len {
                let subs = id_str.as_bytes()
                    .chunks(i)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();
                if subs.iter().all(|s| s == &subs[0] && subs.len() > 1) { chunk_invalid += id; break; }
            }
            if chunk_invalid > 0 {
                println!("{}-{} has {} invalid IDs", start, end, chunk_invalid);
            }
            total += chunk_invalid;
        }
    }

    println!("Adding up all the invalid IDs in this example produces: {}", total);
}
