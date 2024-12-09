use std::usize;

fn parse_input(input: &str) -> Vec<Option<usize>> {
    let mut disk_map: Vec<Option<usize>> = Vec::new();

    input.chars()
        .enumerate()
        .for_each(|(i, c)| {
            let value = if i % 2 == 0 {
                Some(i / 2)
            } else {
                None
            };
            for _ in 0..c.to_digit(10).unwrap() {
                disk_map.push(value);
            }
        });

    disk_map
}

pub fn part1(input: &str) -> usize {
    let mut disk_map = parse_input(input);

    let mut front_idx = 0;
    let mut back_idx = disk_map.len() - 1;

    while front_idx < back_idx {
        if disk_map[front_idx].is_none() {
            while back_idx > front_idx {
                if disk_map[back_idx].is_some() {
                    disk_map.swap(front_idx, back_idx);
                    break;
                }
                back_idx -= 1;
            }
        } else {
            front_idx += 1;
        }
    }

    checksum(&disk_map)
}

fn checksum(disk_map: &Vec<Option<usize>>) -> usize {
    disk_map.iter()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, x)| i * x.unwrap())
        .sum()
}

fn _print_disk_map(disk_map: &Vec<Option<usize>>) {
    println!("{}", disk_map.iter().map(|v| match v {
        Some(id) => format!(" {id} "),
        None => format!(" . "),
    }).collect::<String>());
}

pub fn part2(input: &str) -> usize {
    let mut disk_map = parse_input(input);

    let mut back_idx = disk_map.len() - 1;

    let mut prev_file = usize::MAX;

    while back_idx > 0 {
        let (id, length) = next_file(&disk_map, &mut back_idx);
        if id > prev_file {
           // continue;
        }

        prev_file = id;

        if let Some(to_idx) = find_spot_of_size_before(&disk_map, length, back_idx) {
            for i in 0..length {
                disk_map.swap(to_idx + i + 1, back_idx + i + 1);
            }
        }
    }

    checksum(&disk_map)
}

fn find_spot_of_size_before(disk_map: &Vec<Option<usize>>, required_length: usize, back_idx: usize) -> Option<usize> {
    let mut length = 0;
    for i in 0..back_idx + 1 {
        if disk_map[i] == None {
            length += 1;
            if length == required_length {
                return Some(i - length)
            }
        } else {
            length = 0;
        }
    }
    None
}

fn next_file(disk_map: &Vec<Option<usize>>, back_idx: &mut usize) -> (usize, usize) {
    while disk_map[*back_idx].is_none() {
        *back_idx -= 1;
    }

    let id = disk_map[*back_idx];
    let mut length = 0;
    
    while disk_map[*back_idx] == id && *back_idx > 0 {
        length += 1;
        *back_idx -= 1;
    }

    (id.unwrap(), length)
}