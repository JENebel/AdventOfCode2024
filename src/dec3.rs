use regex::Regex;

pub fn part1(input: &str) -> i32 {
    let regex = Regex::new(r#"mul\([0-9]+,[0-9]+\)"#).unwrap();
    regex.find_iter(input)
        .map(|m| {
            let (a, b) = m.as_str().split_once(",").unwrap();
            let a = &a[4..];
            let b = &b[..b.len() - 1];
            a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
        .sum::<i32>()
}

pub fn part2(input: &str) -> i32 {
    let regex = Regex::new(r#"(mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\))"#).unwrap();
    let mut enabled = true;
    regex.find_iter(input)
        .map(|m| {
            if m.as_str() == "do()" {
                enabled = true;
            } else if m.as_str() == "don't()" {
                enabled = false;
            } else if enabled {
                let (a, b) = m.as_str().split_once(",").unwrap();
                let a = &a[4..];
                let b = &b[..b.len() - 1];
                let r = a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                return r
            }
            0
        })
        .sum::<i32>()
}