use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    solve_n(input, 25)
}

pub fn part2(input: &str) -> usize {
    solve_n(input, 75)
}

fn solve_n(input: &str, n: usize) -> usize {
    let stones: Vec<&str> = input.split(" ").map(|s| s).collect();
    let mut cache = HashMap::new();
    stones.iter().map(|stone| {
        blink_rec(stone.to_string(), n, &mut cache)
    }).sum()
}

fn blink_rec(stone: String, remaining: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    if remaining == 0 {
        return 1
    }

    if let Some(cached) = cache.get(&(stone.clone(), remaining)) {
        return *cached
    }

    let res = if stone == "0" {
        blink_rec("1".to_string(), remaining - 1, cache)
    } else if stone.len() % 2 == 0 {
        let a = &stone[0..stone.len() / 2].parse::<usize>().unwrap();
        let b = &stone[stone.len() / 2..].parse::<usize>().unwrap();
        blink_rec(format!("{a}"), remaining - 1, cache)
        + blink_rec(format!("{b}"), remaining - 1, cache)
    } else {
        blink_rec(format!("{}", stone.parse::<usize>().unwrap() * 2024), remaining - 1, cache)
    };

    cache.insert((stone, remaining), res);

    res
}