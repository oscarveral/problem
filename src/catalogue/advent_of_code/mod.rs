mod year_2023;

use std::ops::RangeInclusive;

use super::{Problem, Solution, Solver};

pub const IDENTIFIER: &str = "advent_of_code";

pub struct AdventOfCodeProblem {
    year: RangeInclusive<usize>,
    day: RangeInclusive<usize>,
    part: RangeInclusive<usize>,
}

pub struct AdventOfCodeSolver {
    solver: fn() -> Solution,
    info: &'static str,
}

impl Problem for AdventOfCodeProblem {
    fn get_solver(&self) -> Vec<AdventOfCodeSolver> {
        self.year
            .clone()
            .into_iter()
            .filter_map(|year| match year {
                2023 => Some(year_2023::get_solver(&self)),
                _ => {
                    eprintln!("Advent of Code: Can't find year {}.", year);
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    fn from_str(s: &str) -> Option<Self> {
        let mut slicing = s.split('/');

        let years = slicing
            .next()?
            .split('-')
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()
            .ok()?;
        let days = slicing
            .next()?
            .split('-')
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()
            .ok()?;
        let parts = slicing
            .next()?
            .split('-')
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()
            .ok()?;

        if slicing.next().is_some() {
            return None;
        }

        let year = match years.len() {
            1 => years[0]..=years[0],
            2 => years[0]..=years[1],
            _ => return None,
        };

        let day = match days.len() {
            1 => days[0]..=days[0],
            2 => days[0]..=days[1],
            _ => return None,
        };

        let part = match parts.len() {
            1 => parts[0]..=parts[0],
            2 => parts[0]..=parts[1],
            _ => return None,
        };

        Some(Self { year, day, part })
    }
}

impl Solver for AdventOfCodeSolver {
    fn solve(&self) -> Solution {
        (self.solver)()
    }

    fn get_info(&self) -> &str {
        self.info
    }
}

macro_rules! create_solver {
    ($name:ident, $solver:expr, $info:literal) => {
        pub const $name: crate::catalogue::advent_of_code::AdventOfCodeSolver =
            crate::catalogue::advent_of_code::AdventOfCodeSolver {
                solver: || $solver().into(),
                info: $info,
            };
    };
}

macro_rules! get_input {
    ($year:expr, $day:expr) => {
        use std::fs::read_to_string;
        pub fn get_input() -> String {
            let file = format!(
                "src/catalogue/advent_of_code/year_{}/inputs/day_{:02}.in",
                $year, $day
            );
            read_to_string(file).expect("Advent of Code: Can't find input file.")
        }
    };
}

macro_rules! get_solver {
    () => {
        use crate::catalogue::advent_of_code::{AdventOfCodeProblem, AdventOfCodeSolver};

        pub fn get_solver(problem: &AdventOfCodeProblem) -> Vec<AdventOfCodeSolver> {
            let problems = problem
                .day
                .clone()
                .map(|day| {
                    problem
                        .part
                        .clone()
                        .into_iter()
                        .map(|part| (day, part))
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<(usize, usize)>>();

            problems
                .into_iter()
                .filter_map(|(day, part)| match (day, part) {
                    (1, 1) => Some(day_01::PART_1),
                    (1, 2) => Some(day_01::PART_2),
                    (2, 1) => Some(day_02::PART_1),
                    (2, 2) => Some(day_02::PART_2),
                    (3, 1) => Some(day_03::PART_1),
                    (3, 2) => Some(day_03::PART_2),
                    (4, 1) => Some(day_04::PART_1),
                    (4, 2) => Some(day_04::PART_2),
                    (5, 1) => Some(day_05::PART_1),
                    (5, 2) => Some(day_05::PART_2),
                    (6, 1) => Some(day_06::PART_1),
                    (6, 2) => Some(day_06::PART_2),
                    (_, 1..=2) => {
                        eprintln!("Advent of Code: Can't find day {}.", day);
                        None
                    }
                    (1..=25, _) => {
                        eprintln!("Advent of Code: Can't find part {} for day {}.", part, day);
                        None
                    }
                    _ => {
                        eprintln!(
                            "Advent of Code: The combination of day {} and part {} is invalid.",
                            day, part
                        );
                        None
                    }
                })
                .collect::<Vec<_>>()
        }
    };
}

pub(crate) use create_solver;
pub(crate) use get_input;
pub(crate) use get_solver;
