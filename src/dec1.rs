pub fn part1(input: &str) -> i32 {
    let (list1, list2) = get_sorted_lists(input);

    let sum: i32 = list1.into_iter().zip(list2).map(|(x, y)| {
        (x - y).abs()
    }).sum();

    sum
}

pub fn part2(input: &str) -> i32 {
    let (list1, list2) = get_sorted_lists(input);

    let mut sum = 0;
    for i in &list1 {
        let mut similarity: i32 = 0;
        for j in &list2 {
            if i == j {
                similarity += 1;
            } else if j > i {
                break;
            }
        }
        sum += similarity * i;
    }
    
    sum
}

fn get_sorted_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let nums = line.split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
        list1.push(nums[0]);
        list2.push(nums[1]);
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}