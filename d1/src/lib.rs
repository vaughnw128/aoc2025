use std::collections::VecDeque;

enum Direction {
    Left,
    Right,
}

// bit of a rotate helper!
fn rotate(dq: &mut VecDeque<u8>, n: usize, dir: Direction) -> u32 {
    let len = dq.len();

    let mut rem_rot = n;
    let mut password_incr: u32 = 0;

    while rem_rot >= len {
        match dir {
            Direction::Left => dq.rotate_right(len),
            Direction::Right => dq.rotate_left(len),
        }
        rem_rot -= len;

        password_incr += 1;
    }

    let final_rot = rem_rot % len;
    if final_rot > 0 {
        let before = dq[50];

        match dir {
            Direction::Left => {
                dq.rotate_right(final_rot);
                if final_rot > before as usize && before != 0 {
                    password_incr += 1;
                }
            }
            Direction::Right => {
                dq.rotate_left(final_rot);
                if final_rot > 100 - before as usize && before != 0 {
                    password_incr += 1;
                }
            }
        }
    }

    password_incr
}

pub fn solve_1(input: &str) -> u32 {
    let mut circular: VecDeque<u8> = VecDeque::from((0..100).collect::<Vec<u8>>());
    let mut password: u32 = 0;

    for line in input.lines() {
        if line.is_empty() { continue; }
        let l_b = line.as_bytes();
        let dir = l_b[0];
        let dist = String::from_utf8(l_b[1..].to_vec()).unwrap().parse::<usize>().unwrap();

        match dir {
            b'L' => {
                rotate(&mut circular, dist, Direction::Left);
            }
            b'R' => {
                rotate(&mut circular, dist, Direction::Right);
            }
            _ => continue,
        }

        if circular[50] == 0 {
            password += 1;
        }
    }

    password
}
pub fn solve_2(input: &str) -> u32 {
    let mut circular: VecDeque<u8> = VecDeque::from((0..100).collect::<Vec<u8>>());
    let mut password: u32 = 0;

    for line in input.lines() {
        if line.is_empty() { continue; }
        let l_b = line.as_bytes();
        let dir = l_b[0];
        let dist = String::from_utf8(l_b[1..].to_vec()).unwrap().parse::<usize>().unwrap();

        match dir {
            b'L' => {
                password += rotate(&mut circular, dist, Direction::Left);
            }
            b'R' => {
                password += rotate(&mut circular, dist, Direction::Right);
            }
            _ => continue,
        }

        if circular[50] == 0 {
            password += 1;
        }
    }

    password
}

