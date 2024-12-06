use std::fmt::{Display, Write};

#[derive(Clone)]
struct Grid {
    inner: Vec<char>,
    width: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut i = 0;
        for c in &self.inner {
            i += 1;
            f.write_char(*c).unwrap();
            if i == self.width {
                i = 0;
                f.write_char('\n').unwrap();
            }
        }
        Ok(())
    }
}

impl Grid {
    pub fn get(&self, idx: usize) -> char {
        self.inner[idx]
    }

    pub fn set(&mut self, idx: usize, new: char) {
        self.inner[idx] = new;
    }

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
                if from + self.width > self.inner.len() {
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

#[derive(Copy, Clone, PartialEq, Debug)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn parse_input(input: &str) -> (Grid, usize) {
    let width = input.split_once("\n").unwrap().0.len();
    let mut start = 0;
    let inner: Vec<char> = input.lines()
                                .flat_map(|s| s.chars())
                                .enumerate()
                                .map(|(i, c)| {
                                    if c == '^' {
                                        start = i;
                                    }
                                    c
                                })
                                .collect();
    let grid = Grid {inner, width};
    (grid, start)
}

pub fn part1(input: &str) -> usize {
    let (mut grid, start) = parse_input(input);
    grid.set(start, 'X');

    let mut pos = start;
    let mut dir = Direction::Up;
    while let Some(next_pos) = grid.move_dir(dir, pos) {
        if grid.get(next_pos) == '#' {
            dir = dir.rotate_right();
        } else {
            grid.set(next_pos, 'X');
            pos = next_pos
        }
    }
    grid.inner.iter().filter(|c| **c == 'X').count()
}

pub fn part2(input: &str) -> usize {
    let (mut grid, start) = parse_input(input);
    grid.set(start, 'X');

    let mut pos = start;
    let mut dir = Direction::Up;
    while let Some(next_pos) = grid.move_dir(dir, pos) {
        if grid.get(next_pos) == '#' {
            dir = dir.rotate_right();
        } else {
            if grid.get(next_pos) != 'O'{
                grid.set(next_pos, '#');
                if is_cycle(&grid, start) {
                    grid.set(next_pos, 'O');
                } else {
                    grid.set(next_pos, 'X');
                }
            }
            pos = next_pos
        }
    }
    grid.inner.iter().filter(|c| **c == 'O').count()
}

fn is_cycle(grid: &Grid, start_pos: usize) -> bool {
    let mut pos = start_pos;
    let mut dir = Direction::Up;
    let mut seen = vec![];
    while let Some(next_pos) = grid.move_dir(dir, pos) {
        if grid.get(next_pos) == '#' {
            dir = dir.rotate_right();
        } else {
            if seen.contains(&(next_pos, dir)) {
                return true
            }
            pos = next_pos;
        }

        seen.push((pos, dir));
    }
    false
}