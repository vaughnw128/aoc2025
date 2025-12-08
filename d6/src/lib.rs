pub fn solve_1(input: &str) -> u64 {
    let mut operators = Vec::new();
    let mut lines = input.lines().collect::<Vec<&str>>();
    operators = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>();

    let nums = lines.iter().map(|line| line.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect::<Vec<u64>>()).collect::<Vec<Vec<u64>>>();

    let mut pivoted: Vec<Vec<u64>> = Vec::new();
    for j in 0..nums[0].len() {
        let mut new_row: Vec<u64> = Vec::new();
        for i in 0..nums.len() {
            new_row.push(nums[i][j]);
        }
        pivoted.push(new_row);
    }

    let mut total: u64 = 0;
    for (i, line) in pivoted.iter().enumerate() {
        let res: u64 = match operators[i] {
            "+" => {
                line.iter().sum()
            }
            "*" => {
                line.iter().product()
            }
            _ => panic!("Invalid operator"),
        };

        total += res;
    }

    total
}

pub fn solve_2(input: &str) -> u64 {
    let mut operators = Vec::new();
    let mut lines = input.lines().collect::<Vec<&str>>();
    operators = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .rev()
        .collect::<Vec<&str>>();

    let max_line_len = lines.iter().map(|l| l.len()).max().unwrap();
    let nums = lines
        .iter()
        .map(|line| {
            format!("{}{}", line, " ".repeat(max_line_len - line.len()))
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut pivoted: Vec<Vec<char>> = Vec::new();
    for j in 0..nums[0].len() {
        let mut new_row: Vec<char> = Vec::new();
        for i in 0..nums.len() {
            new_row.push(nums[i][j]);
        }
        pivoted.push(new_row);
    }
    pivoted.insert(0, vec![' '; pivoted[0].len()]);

    let mut sum = 0;
    let mut count = 0;
    let mut register_nums = Vec::new();
    for line in pivoted.iter().rev() {
        if line.iter().all(|n| *n == ' ') {
            let op = operators[count];
            if op == "+" {
                sum += register_nums.iter().sum::<u64>();
                register_nums.clear();
                count += 1;
            } else {
                let mut product = 1;
                for n in register_nums.iter() {
                    product *= *n;
                }
                sum += product;
                register_nums.clear();
                count += 1;
            }
        } else {
            register_nums.push(
                line.iter()
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap(),
            );
        }
    }

    sum
}