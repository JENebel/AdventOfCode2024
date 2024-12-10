use std::collections::HashSet;

#[derive(Clone)]
struct Grid {
    inner: Vec<u8>,
    width: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up, Down, Left, Right
}

impl Grid {
    pub fn move_dir(&self, dir: Direction, from: usize) -> Option<usize> {
        match dir {
            Direction::Up => {
                if from < self.width {
                    None
                } else {
                    Some(from - self.width)
                }
            },
            Direction::Down => {
                if from + self.width + 1 > self.inner.len() {
                    None
                } else {
                    Some(from + self.width)
                }
            },
            Direction::Left => {
                if from % self.width > 0 {
                    Some(from - 1)
                } else {
                    None
                }
            },
            Direction::Right => {
                if from % self.width < self.width - 1 {
                    Some(from + 1)
                } else {
                    None
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let width = input.split_once("\n").unwrap().0.len();
    let inner: Vec<u8> = input.lines()
                                .flat_map(|s| s.chars())
                                .map(|c| {
                                    c.to_digit(10).unwrap() as u8
                                })
                                .collect();
    let grid = Grid {inner, width};
    grid
}

fn search(grid: &Grid, head: usize) -> usize {
    let mut seen = HashSet::new();
    let mut queue = HashSet::new();
    queue.insert(head);
    for height in 1..=9 {
        let mut new_queue = HashSet::new();
        for queued in &queue {
            [
                grid.move_dir(Direction::Left, *queued),
                grid.move_dir(Direction::Up, *queued),
                grid.move_dir(Direction::Right, *queued),
                grid.move_dir(Direction::Down, *queued)
            ].iter()
                .filter_map(|idx| idx.map(|idx| idx))
                .filter(|idx| grid.inner[*idx] == height)
                .for_each(|idx| 
                    if !seen.contains(&idx) {
                        new_queue.insert(idx);
                        seen.insert(idx);
                    }
                );
        }

        if height == 9 {
            return new_queue.len()
        } else {
            queue.clear();
        }

        for elem in new_queue {
            queue.insert(elem);
        }
    }

    unreachable!()
}

pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    grid.inner
        .iter()
        .enumerate()
        .filter(|(_, height)| **height == 0)
        .map(|(i, _)| search(&grid, i))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    grid.inner
        .iter()
        .enumerate()
        .filter(|(_, height)| **height == 0)
        .map(|(i, _)| search2(&grid, i))
        .sum()
}

fn search2(grid: &Grid, head: usize) -> usize {
    let mut queue = Vec::new();
    queue.push(head);
    for height in 1..=9 {
        let mut new_queue = Vec::new();
        for queued in &queue {
            [
                grid.move_dir(Direction::Left, *queued),
                grid.move_dir(Direction::Up, *queued),
                grid.move_dir(Direction::Right, *queued),
                grid.move_dir(Direction::Down, *queued)
            ].iter()
                .filter_map(|idx| idx.map(|idx| idx))
                .filter(|idx| grid.inner[*idx] == height)
                .for_each(|idx| 
                    new_queue.push(idx)
                );
        }

        if height == 9 {
            return new_queue.len()
        } else {
            queue.clear();
        }

        for elem in new_queue {
            queue.push(elem);
        }
    }

    unreachable!()
}