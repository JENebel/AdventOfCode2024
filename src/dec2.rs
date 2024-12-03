use std::fmt::Display;

pub fn part1(input: &str) -> impl Display {
    input.lines().filter(|line| {
        let chars: Vec<i32> = line.split_whitespace()
                                  .map(|s| s.parse().unwrap())
                                  .collect();
        is_valid(chars)

    }).count() as i32
}

pub fn part2(input: &str) -> impl Display {
    input.lines().filter(|line| {
        let chars: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        is_valid(chars.clone()) || (0..chars.len()).any(|i| {
            let slice = [&chars[0..i], &chars[i + 1..]].concat().clone();
            is_valid(slice)
        })
    }).count() as i32
}

fn is_valid(mut chars: Vec<i32>) -> bool {
    if !chars.is_sorted() {
        chars.reverse();
        if !chars.is_sorted() {
            return false;
        }
    }

    !chars.windows(2).any(|w| 
        w[0].abs_diff(w[1]) == 0 || 
        w[0].abs_diff(w[1]) > 3)
}