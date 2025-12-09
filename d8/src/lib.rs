use std::collections::HashMap;

fn get_distance(jb_1: &JunctionBox, jb_2: &JunctionBox) -> f64 {
    (jb_1.x as f64 - jb_2.x as f64).powf(2.0)
        + (jb_1.y as f64 - jb_2.y as f64).powf(2.0)
        + (jb_1.z as f64 - jb_2.z as f64).powf(2.0)
}

#[derive(Eq, Hash, Debug, Clone, PartialEq)]
#[derive(PartialOrd)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    fn new(x: usize, y: usize, z: usize) -> Self {
        JunctionBox { x, y, z }
    }
}

// get index of chain containing junction box
fn find_chain_index(chains: &Vec<Vec<JunctionBox>>, jb: &JunctionBox) -> Option<usize> {
    chains.iter().position(|c| c.contains(jb))
}

fn try_merge_chains(
    chains: &mut Vec<Vec<JunctionBox>>,
    jb_1: &JunctionBox,
    jb_2: &JunctionBox,
) -> bool {
    let c_i_1 = find_chain_index(chains, jb_1);
    let c_i_2 = find_chain_index(chains, jb_2);

    match (c_i_1, c_i_2) {
        (Some(a), Some(b)) => {
            if a != b {
                let (p1, p2) = if a < b { (a, b) } else { (b, a) };
                let mut moved = chains.remove(p2);
                for item in moved.drain(..) {
                    if !chains[p1].contains(&item) {
                        chains[p1].push(item);
                    }
                }
                return true
            }
            false
        }
        _ => false,
    }
}

pub fn solve_1(input: &str, size: usize) -> u128 {
    let mut pool: Vec<JunctionBox> = Vec::new();

    for line in input.lines() {
        let loc = line.split(",").map(|s|s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        pool.push(JunctionBox::new(loc[0], loc[1], loc[2]))
    }

    // grab all unique pairs of junction boxes
    let mut pairs = HashMap::new();
    for i in 0..pool.len() {
        for j in 0..pool.len() {
            if i != j && !pairs.contains_key(&(pool[j].clone(), pool[i].clone())) {
                pairs.insert((pool[i].clone(), pool[j].clone()), get_distance(&pool[i], &pool[j]));
            }
        }
    }

    // sort the pairs by distance
    let mut sorted_pairs = pairs.into_iter().collect::<Vec<((JunctionBox, JunctionBox), f64)>>();
    sorted_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut chains: Vec<Vec<JunctionBox>> = Vec::new();
    for ((jb_1, jb_2), _dist) in sorted_pairs[0..size].iter() {
        // attempt inline merge if both junctions already in different chains
        if try_merge_chains(&mut chains, jb_1, jb_2) {
            continue;
        }

        if let Some(c) = chains.iter_mut().find(|c| {
            c.contains(&jb_1) || c.contains(&jb_2)
        }) {
            if !c.contains(&jb_1) {
                c.push(jb_1.clone());
            }
            if !c.contains(&jb_2) {
                c.push(jb_2.clone());
            }
        } else {
            chains.push(vec![jb_1.clone(), jb_2.clone()]);
        }
    }

    // get size of top 3 largest chains
    chains.sort_by(|a, b| b.len().cmp(&a.len()));
    let tot = chains[0..3].iter().map(|c| c.len() as u32).product::<u32>();

    tot as u128
}

pub fn solve_2(input: &str) -> u128 {
    let mut pool: Vec<JunctionBox> = Vec::new();

    for line in input.lines() {
        let loc = line.split(",").map(|s|s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        pool.push(JunctionBox::new(loc[0], loc[1], loc[2]))
    }

    // grab all unique pairs of junction boxes
    let mut pairs = HashMap::new();
    for i in 0..pool.len() {
        for j in 0..pool.len() {
            if i != j && !pairs.contains_key(&(pool[j].clone(), pool[i].clone())) {
                pairs.insert((pool[i].clone(), pool[j].clone()), get_distance(&pool[i], &pool[j]));
            }
        }
    }

    // sort the pairs by distance
    let mut sorted_pairs = pairs.into_iter().collect::<Vec<((JunctionBox, JunctionBox), f64)>>();
    sorted_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut chains: Vec<Vec<JunctionBox>> = Vec::new();

    for ((jb_1, jb_2), _dist) in sorted_pairs.iter() {
        // attempt inline merge if both junctions already in different chains

        if try_merge_chains(&mut chains, jb_1, jb_2) {
            if chains.len() == 1 && chains[0].len() == pool.len() {
                return (jb_1.x * jb_2.x) as u128
            }
            continue;
        }

        if let Some(c) = chains.iter_mut().find(|c| {
            c.contains(&jb_1) || c.contains(&jb_2)
        }) {
            if !c.contains(&jb_1) {
                c.push(jb_1.clone());
            }
            if !c.contains(&jb_2) {
                c.push(jb_2.clone());
            }
        } else {
            chains.push(vec![jb_1.clone(), jb_2.clone()]);
        }

        if chains.len() == 1 && chains[0].len() == pool.len() {
            return (jb_1.x * jb_2.x) as u128
        }
    }

    0
}
