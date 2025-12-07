pub fn solve_1(input: &str) -> usize {
    let mut total: usize = 0;

    for (start, end) in input
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let nums = s
                .split('-')
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (nums[0], nums[1])
        })
    {
        for id in start..=end {
            let id_str = id.to_string();
            let l = id_str.len();

            if l % 2 != 0 { continue; }

            let chunks = id_str
                .chars()
                .collect::<Vec<char>>()
                .chunks(l/2)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();

            if chunks[0] != chunks[1] { continue; }
            
            total += id_str.parse::<usize>().unwrap();
        }
    }

    total
}

pub fn solve_2(input: &str) -> usize {
    let mut total: usize = 0;

    for (start, end) in input
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| {
            let nums = s
                .split('-')
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (nums[0], nums[1])
        })
    {
        for id in start..=end {
            let id_str = id.to_string();
            let l = id_str.len();
            let max_len = if l > 2 { l / 2 } else { l };

            let mut chunk_invalid = 0;
            for i in 1..=max_len {
                let subs = id_str
                    .as_bytes()
                    .chunks(i)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();
                if subs.iter().all(|s| s == &subs[0] && subs.len() > 1) {
                    chunk_invalid += id;
                    break;
                }
            }
            total += chunk_invalid;
        }
    }

    total
}

