pub fn part1(input: &str) -> i32 {
    let len = input.split_once("\n").unwrap().0.len() as isize;
    let vec: Vec<char> = input.chars().filter(|c| *c != '\n').collect();

    let mut result = 0;
    for i in 0..vec.len() {
        if vec[i] != 'X' {
            continue
        }

        result += explore_direction(&vec, i, len, -len - 1);
        result += explore_direction(&vec, i, len, -len);
        result += explore_direction(&vec, i, len, -len + 1);
        result += explore_direction(&vec, i, len, len - 1);
        result += explore_direction(&vec, i, len, len);
        result += explore_direction(&vec, i, len, len + 1);
        result += explore_direction(&vec, i, len, -1);
        result += explore_direction(&vec, i, len, 1);
    }

    result
}

fn explore_direction(vec: &Vec<char>, pos: usize, len: isize, offset: isize) -> i32 {
    let mut pos = pos as isize;
    for c in "MAS".chars() {
        pos += offset;
        if pos < 0 || pos >= vec.len() as isize || (pos % len).abs_diff((pos - offset) % len) > 1 {
            return 0;
        }

        if c != vec[pos as usize] {
            return 0;
        }
    }
    1
}

pub fn part2(input: &str) -> i32 {
    let len = input.split_once("\n").unwrap().0.len() as isize;
    let vec: Vec<char> = input.chars().filter(|c| *c != '\n').collect();

    let mut result = 0;
    for i in 0..vec.len() {
        if vec[i] != 'A' {
            continue
        }

        let Some(top_left) = get_cell(&vec, i as isize, len, -len - 1) else {
            continue
        };
        let Some(top_right) = get_cell(&vec, i as isize, len, -len + 1) else {
            continue
        };
        let Some(bot_left) = get_cell(&vec, i as isize, len, len - 1) else {
            continue
        };
        let Some(bot_right) = get_cell(&vec, i as isize, len, len + 1) else {
            continue
        };

        if top_left == 'M' && top_right == 'M' && bot_left == 'S' && bot_right == 'S' ||
           top_left == 'M' && top_right == 'S' && bot_left == 'M' && bot_right == 'S' ||
           top_left == 'S' && top_right == 'M' && bot_left == 'S' && bot_right == 'M' ||
           top_left == 'S' && top_right == 'S' && bot_left == 'M' && bot_right == 'M' {
            result += 1;
        }
    }

    result
}

pub fn get_cell(vec: &Vec<char>, pos: isize, len: isize, offset: isize) -> Option<char> {
    let pos = pos + offset;
    if pos < 0 || pos >= vec.len() as isize || (pos % len).abs_diff((pos - offset) % len) > 1 {
        None
    } else {
        Some(vec[pos as usize])
    }
}