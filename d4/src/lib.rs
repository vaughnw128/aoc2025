fn run_simulation(vec_2d: &mut Vec<Vec<bool>>, count: &mut u128, clear: bool) {
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
                    if clear { vec_2d[i][j] = false; }
                    *count += 1;
                }
            }
        }
    }
}

fn print_2d(vec_2d: &Vec<Vec<bool>>) {
    for row in vec_2d.iter() {
        for cell in row.iter() {
            print!("{}", if *cell { "@" } else { "." });
        }
        println!();
    }
}

pub fn solve_1(input: &str) -> u128 {
    let mut vec_2d: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
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

    let mut count = 0;

    run_simulation(&mut vec_2d, &mut count, false);

    count
}

pub fn solve_2(input: &str) -> u128 {
    let mut vec_2d: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
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
        run_simulation(&mut vec_2d, &mut count, true);

        // set the prev iteration and continue
        if prev_iteration == vec_2d { break } else {
            prev_iteration = vec_2d.clone();
        }
    }

    count
}
