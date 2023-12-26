use std::fmt::{Display, Formatter, Result};

use Problem::*;

const FIRST_SEPARATOR: char = ':';
const SECOND_SEPARATOR: char = '-';
const PROBLEM_AOC: &str = "advent-of-code";

pub enum Problem {
    AdventOfCode(usize, usize, usize),
    None,
}

pub fn get_problems(args: Vec<String>) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    for arg in args {
        let problem = Problem::from(arg.as_str());
        if problem.is_none() {
            eprintln!("Invalid problem: {}", arg);
            continue;
        }
        problems.push(problem);
    }
    problems
}

impl Problem {
    pub fn is_none(&self) -> bool {
        matches!(self, None)
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AdventOfCode(year, day, part) => {
                write!(f, "Advent of Code {} Day {} Part {}", year, day, part)
            }
            None => write!(f, "None"),
        }
    }
}

impl From<&str> for Problem {
    fn from(s: &str) -> Self {
        let problem: Vec<&str> = s.split(FIRST_SEPARATOR).collect();
        if problem.len() != 2 {
            eprintln!(
                "Invalid problem format: {}. Use ':' as separator: problem_type:problem_id.",
                s
            );
        }
        match problem[0] {
            PROBLEM_AOC => parse_advent_of_code(problem[1]),
            _ => {
                eprintln!("Can't find the problem type for: {}", s);
                None
            }
        }
    }
}

fn parse_advent_of_code(s: &str) -> Problem {
    let problem: Vec<&str> = s.split(SECOND_SEPARATOR).collect();
    if problem.len() != 3 {
        eprintln!(
            "Invalid problem format: {}. Use '-' as separator: year-day-part.",
            s
        );
        return None;
    }
    let year = problem[0].parse::<usize>();
    let day = problem[1].parse::<usize>();
    let part = problem[2].parse::<usize>();
    if year.is_err() || day.is_err() || part.is_err() {
        eprintln!(
            "Can't parse problem data: {}. Problem id must contain only numbers.",
            s
        );
        return None;
    }
    AdventOfCode(year.unwrap(), day.unwrap(), part.unwrap())
}
