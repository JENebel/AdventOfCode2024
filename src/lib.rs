use std::fmt::Display;

use paste::paste;

import_solver!(1);
import_solver!(2);
import_solver!(3);
import_solver!(4);
import_solver!(5);

pub trait Solver<const DAY: u8> {
    fn part1(input: &str) -> impl Display;
    fn part2(input: &str) -> impl Display;

    fn solve_part_1() {
        let input = std::fs::read_to_string(format!("inputs/dec{}.input", DAY)).unwrap();

        let before = std::time::Instant::now();
        let solution = Self::part1(&input);
        let after = std::time::Instant::now();
        println!("\r Part 1: {} - Took {:?}", solution, after.duration_since(before));
    }

    fn solve_part_2() {
        let input = std::fs::read_to_string(format!("inputs/dec{}.input", DAY)).unwrap();

        let before = std::time::Instant::now();
        let solution = Self::part2(&input);
        let after = std::time::Instant::now();
        println!("\r Part 2: {} - Took {:?}", solution, after.duration_since(before));
    }

    fn solve() {
        println!("Day {}", DAY);
        Self::solve_part_1();
        Self::solve_part_2();
    }
}

#[macro_export]
macro_rules! import_solver {
    ($day:expr) => {
        paste! {
            mod [<dec $day>];
            pub struct [<Dec $day>] {}
            impl crate::Solver<$day> for [<Dec $day>] {
                fn part1(input: &str) -> impl std::fmt::Display {
                    [<dec $day>]::part1(input)
                }
                
                fn part2(input: &str) -> impl std::fmt::Display {
                    [<dec $day>]::part2(input)
                }
            }
        }
    };
}