// https://adventofcode.com/2025/day/4

use std::env;
use std::fs::read_to_string;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = &args[1];

    let mut vec_2d: Vec<Vec<bool>> = Vec::new();
    for line in read_to_string(input).unwrap().lines() {
        vec_2d.push(line.chars().map(|c| c == '@').collect::<Vec<bool>>());
    }

    // Add walls around the 2d vec so we can more easily check neighbors
    let width = vec_2d[0].len();
    let wall_row = vec![false; width];
    vec_2d.insert(0, wall_row.clone());
    vec_2d.push(wall_row);
    for row in vec_2d.iter_mut() {
        row.insert(0, false);
        row.push(false);
    }

    // prev iteration to check for stabalization
    let mut prev_iteration = vec_2d.clone();
    let mut count = 0;
    loop {
        for i in 0..vec_2d.len() {
            for j in 0..vec_2d.len() {
                if *vec_2d.get(i).unwrap().get(j).unwrap() {
                    // check neighbors
                    let neighbors = vec![
                        vec_2d.get(i - 1).unwrap().get(j - 1).unwrap(),
                        vec_2d.get(i - 1).unwrap().get(j).unwrap(),
                        vec_2d.get(i - 1).unwrap().get(j + 1).unwrap(),
                        vec_2d.get(i).unwrap().get(j - 1).unwrap(),
                        vec_2d.get(i).unwrap().get(j + 1).unwrap(),
                        vec_2d.get(i + 1).unwrap().get(j - 1).unwrap(),
                        vec_2d.get(i + 1).unwrap().get(j).unwrap(),
                        vec_2d.get(i + 1).unwrap().get(j + 1).unwrap(),
                    ];
                    let neighbor_count = neighbors.iter().filter(|n| ***n).count();

                    if neighbor_count < 4 {
                        vec_2d[i][j] = false;
                        count += 1;
                    }
                }
            }
        }

        // set the prev iteration and continue
        if prev_iteration == vec_2d { break } else {
            prev_iteration = vec_2d.clone();
        }
    }

    println!("Stabalized! Total: {}", count);
}
