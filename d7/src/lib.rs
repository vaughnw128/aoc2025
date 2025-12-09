use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
enum GridElement {
    Splitter,
    Empty,
    Beam,
    Start
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().map(|c| match c {
        '.' => GridElement::Empty,
        '|' => GridElement::Beam,
        '^' => GridElement::Splitter,
        'S' => GridElement::Start,
        _ => panic!("Invalid character: {}", c)
    }).collect()).collect()
}

type Grid = Vec<Vec<GridElement>>;


// grab the first start location panicking if there isn't one (will not happen ever ever)
fn get_start(grid: &Grid) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if let GridElement::Start = elem {
                return (i, j);
            }
        }
    }
    panic!("No start found");
}

struct Beam {
    location: (usize, usize)
}

#[derive(Debug, Clone)]
struct LocationError;

// returns the next location and the element at that location
// returns an error if the location is out of bounds
// lets us control end state easily
fn next_loc(loc: (usize, usize), grid: &Grid) -> Result<((usize, usize), &GridElement), LocationError> {
    let elem = &grid.get(loc.0 + 1).ok_or(LocationError)?
        .get(loc.1).ok_or(LocationError)?;
    Ok(((loc.0 + 1, loc.1), elem))
}

// beam handler, makes recursion a lot easier
impl Beam {
    fn new(x: usize, y: usize, grid: &mut Grid) -> Self {
        grid[x][y] = GridElement::Beam;
        Beam { location: (x, y) }
    }

    // we don't need mutable grid access here, so we make it immutable
    fn new_immutable(x: usize, y: usize) -> Self {
        Beam { location: (x, y) }
    }

    fn step_solution1(&mut self, grid: &mut Grid, splits: &mut u32) {
        // recurse infinitely until the end is nigh
        loop {
            // print_grid(&grid);
            let (next_loc, elem) = match next_loc(self.location, grid) {
                Ok((loc, elem)) => (loc, elem),
                Err(_) => return // we break after the end row
            };

            // handle beam interaction
            match elem {
                GridElement::Empty => {
                    grid[next_loc.0][next_loc.1] = GridElement::Beam;
                    self.location = next_loc;
                },
                GridElement::Splitter => {
                    *splits += 1;
                    let mut beam1 = Beam::new(next_loc.0, next_loc.1 + 1, grid);
                    beam1.step_solution1(grid, splits);
                    let mut beam2 = Beam::new(next_loc.0, next_loc.1 - 1, grid);
                    beam2.step_solution1(grid, splits);
                    return
                    // we break after all of the beams are finished
                },
                GridElement::Beam => return,
                GridElement::Start => panic!("Start encountered")
            }
        }
    }

    fn step_solution2(&mut self, grid: &Grid, memory: &mut HashMap<(usize, usize), u128>) -> u128 {
        // recurse infinitely until the end is nigh
        loop {
            // don't waste our damn time
            if let Some(&timelines) = memory.get(&self.location) {
                return timelines;
            }

            // advance advance advance
            let (next_loc, elem) = match next_loc(self.location, grid) {
                Ok((loc, elem)) => (loc, elem),
                Err(_) => {
                    memory.insert(self.location, 1);
                    return 1; // hit end
                }
            };

            // handle beam interaction
            match elem {
                GridElement::Empty | GridElement::Beam => {
                    // continue straight down
                    self.location = next_loc;
                    // loop continues until split or end
                }
                GridElement::Splitter => {
                    // branch to both left and right timelines
                    let mut beam1 = Beam::new_immutable(next_loc.0, next_loc.1 + 1);
                    let count_1 = beam1.step_solution2(grid, memory);
                    let mut beam2 = Beam::new_immutable(next_loc.0, next_loc.1 - 1);
                    let count_2 = beam2.step_solution2(grid, memory);
                    let total = count_1 + count_2;

                    // insert into the memory
                    memory.insert(self.location, total);
                    return total;
                }
                GridElement::Start => panic!("Start encountered"),
            }
        }
    }
}

pub fn solve_1(input: &str) -> u128 {
    let mut grid = parse_input(&input);

    let start_loc = get_start(&grid);
    let mut beam = Beam::new_immutable(start_loc.0, start_loc.1);

    let mut splits = 0;
    beam.step_solution1(&mut grid, &mut splits);

    splits as u128
}

pub fn solve_2(input: &str) -> u128 {
    let grid = parse_input(&input);
    
    let start_loc = get_start(&grid);
    let mut beam = Beam::new_immutable(start_loc.0, start_loc.1);

    let timelines = beam.step_solution2(&grid, &mut HashMap::new());

    timelines
}

