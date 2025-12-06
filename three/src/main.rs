// https://adventofcode.com/2025/day/3

use std::fs::read_to_string;


fn main() {
    let mut sum = 0;
    for line in read_to_string("input").unwrap().lines() {
        let mut bank = line.chars().map(|c| c as u8 - '0' as u8).collect::<Vec<u8>>();

        let mut new_bank = vec![0; bank.len()];
        let mut cursor = 0;
        for i in (bank.len()-11..bank.len()) {

        }
    }

}
