fn parse_input<'a>(input: &'a str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(|line| {
        let (target, nums) = line.split_once(":").unwrap();
        let target = target.parse().unwrap();
        let nums = nums.trim()
                       .split(' ')
                       .map(|n| n.parse().unwrap())
                       .collect();
        (target, nums)
    }).collect()
}

pub fn part1(input: &str) -> u64 {
    parse_input(input).iter().filter(|x| is_possible_add_mul(x.0, &x.1)).map(|x| x.0).sum()
}

fn is_possible_add_mul(target: u64, nums: &[u64]) -> bool {
    match nums {
        [a] => *a == target,
        nums => {
            is_possible_add_mul(target, &[&[nums[0] + nums[1]], &nums[2..]].concat()) ||
            is_possible_add_mul(target, &[&[nums[0] * nums[1]], &nums[2..]].concat())
        }
    }
}

pub fn part2(input: &str) -> u64 {
    parse_input(input).iter().filter(|x| is_possible_add_mul_comb(x.0, &x.1)).map(|x| x.0).sum()
}

fn is_possible_add_mul_comb(target: u64, nums: &[u64]) -> bool {
    match nums {
        [a] => *a == target,
        nums => {
            is_possible_add_mul_comb(target, &[&[nums[0] + nums[1]], &nums[2..]].concat()) ||
            is_possible_add_mul_comb(target, &[&[nums[0] * nums[1]], &nums[2..]].concat()) ||
            is_possible_add_mul_comb(target, &[&[format!("{}{}", nums[0], nums[1]).parse().unwrap()], &nums[2..]].concat())
        }
    }
}