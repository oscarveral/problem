super::get_input!(2023, 1);

super::create_solver!(
    PART_1,
    sum_of_all,
    "Advent of Code 2023, Day 1, Part 1. Trebuchet?!"
);
super::create_solver!(
    PART_2,
    sum_of_all_exhaustive,
    "Advent of Code 2023, Day 1, Part 2. Trebuchet?!"
);

fn sum_of_all() -> u32 {
    let input = get_input();

    input
        .lines()
        .filter_map(|line| {
            let mut nums = line.chars().filter_map(|c| c.to_digit(10));
            let first = nums.next();
            let last = nums.next_back().or(first);
            first.zip(last).map(|(first, last)| first * 10 + last)
        })
        .sum::<u32>()
}

fn sum_of_all_exhaustive() -> u32 {
    let input = get_input();

    let digit_strs = [
        ("one", 1u32),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    input
        .lines()
        .filter_map(|line| {
            let first = (0..line.len()).find_map(|start| {
                digit_strs.iter().find_map(|&(digit_str, val)| {
                    if line[start..].starts_with(digit_str) {
                        Some(val)
                    } else {
                        None
                    }
                })
            });
            let last = (0..line.len()).rev().find_map(|end| {
                digit_strs.iter().find_map(|&(digit_str, val)| {
                    if line[..=end].ends_with(digit_str) {
                        Some(val)
                    } else {
                        None
                    }
                })
            });
            first.zip(last).map(|(a, b)| a * 10 + b)
        })
        .sum::<u32>()
}
