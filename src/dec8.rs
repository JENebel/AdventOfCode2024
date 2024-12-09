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
    fn get_xy(&self, idx: isize, delta_x: isize, delta_y: isize) -> (isize, isize) {
        let width = self.width as isize;

        let x = (idx % width) + delta_x;
        let y = (idx / width) + delta_y;

        (x, y)
    }

    fn get_cell(&self, x: isize, y: isize) -> Option<usize> {
        let width = self.width as isize;
        let height = (self.inner.len() / self.width) as isize;
        
        if x >= 0 && x < width && y >= 0 && y < height {
            Some((y * width + x) as usize)
        } else {
            None
        }
    }
    
    pub fn set(&mut self, idx: usize, new: char) {
        self.inner[idx] = new;
    }
}

fn parse_input(input: &str) -> Grid {
    let width = input.split_once("\n").unwrap().0.len();
    let inner: Vec<char> = input.lines()
                                .flat_map(|s| s.chars())
                                .collect();
    Grid {inner, width}
}

pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let mut frequencies: Vec<char> = grid.inner.clone().into_iter().filter(|c| *c != '.').collect();
    frequencies.dedup();

    let mut out_grid = Grid {
        inner: vec!['.'; grid.inner.len()],
        width: grid.width
    };

    frequencies.iter().for_each(|freq| add_hotspots(&grid, &mut out_grid, *freq));

    out_grid.inner.iter().filter(|c| **c == 'X').count()
}

fn add_hotspots(grid: &Grid, out_grid: &mut Grid, freq: char) {
    let positions: Vec<isize> = grid.inner.iter().enumerate().filter(|(_, c)| **c == freq).map(|(i, _)| i as isize).collect();
    let width = grid.width as isize;

    for (i, a) in positions.iter().enumerate() {
        for b in &positions[i + 1..] {
            let x_diff = a % width -  b % width;
            let y_diff = a / width - b / width;

            let first = grid.get_xy(*a, x_diff, y_diff);
            if let Some(idx) = grid.get_cell(first.0, first.1) {
                out_grid.set(idx, 'X');
            }

            let second = grid.get_xy(*b, -x_diff, -y_diff);
            if let Some(idx) = grid.get_cell(second.0, second.1) {
                out_grid.set(idx, 'X');
            }
        }
    }
}

pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let mut frequencies: Vec<char> = grid.inner.clone().into_iter().filter(|c| *c != '.').collect();
    frequencies.dedup();

    let mut out_grid = Grid {
        inner: vec!['.'; grid.inner.len()],
        width: grid.width
    };

    frequencies.iter().for_each(|freq| add_hotspots2(&grid, &mut out_grid, *freq));

    out_grid.inner.iter().filter(|c| **c == 'X').count()
}

fn add_hotspots2(grid: &Grid, out_grid: &mut Grid, freq: char) {
    let positions: Vec<isize> = grid.inner.iter().enumerate().filter(|(_, c)| **c == freq).map(|(i, _)| i as isize).collect();
    let width = grid.width as isize;

    for (i, a) in positions.iter().enumerate() {
        for b in &positions[i + 1..] {
            let x_diff = a % width -  b % width;
            let y_diff = a / width - b / width;

            let gcd = gcd(x_diff.abs(), y_diff.abs());

            let off_x = x_diff / gcd;
            let off_y = y_diff / gcd;

            let mut i = 0;

            while let Some(idx) = {
                let xy = grid.get_xy(*a, off_x * i, off_y * i);
                grid.get_cell(xy.0, xy.1)
            } {
                i += 1;
                out_grid.set(idx, 'X');
            }

            i = 0;

            while let Some(idx) = {
                let xy = grid.get_xy(*a, -off_x * i, -off_y * i);
                grid.get_cell(xy.0, xy.1)
            } {
                i += 1;
                out_grid.set(idx, 'X');
            }
        }
    }
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t
    }
    a
}