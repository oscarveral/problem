super::get_input!(2023, 9);

super::create_solver!(PART_1, sum_diffs, "Advent of Code 2023, Day 9, Part 1. Mirage Maintenance.");
super::create_solver!(PART_2, sum_rev_diffs, "Advent of Code 2023, Day 9, Part 2. Mirage Maintenance.");

fn extrapolate(history: Vec<i64>) -> i64 {
	let Some(&last) = history.last() else {
        return 0;
    };	
	let mut diffs = history;
	let mut acc = 0;

	while diffs.len() > 1 {
		let mut zero = true;
		for i in 1..diffs.len() {
			let diff = diffs[i] - diffs[i - 1];
			if diff != 0 {
				zero = false;
			}
			diffs[i - 1] = diff;
			if i == diffs.len() - 1 {
				acc += diff;
			}
		}
		if zero {
			break;
		}
		diffs.pop();
	}
	last + acc
}

fn sum_diffs() -> i64 {
	let input = get_input();

	input.lines().map(|line| {
		let nums = line.split_whitespace()
			.map(|s| s.parse::<i64>().unwrap_or_else(|_| panic!("Advent of Code: Failed to parse input.")))
			.collect::<Vec<_>>();
		extrapolate(nums)
	}).sum()
}

fn sum_rev_diffs() -> i64 {
	let input = get_input();

	input.lines().map(|line| {
		let nums = line.split_whitespace()
			.rev()
			.map(|s| s.parse::<i64>().unwrap_or_else(|_| panic!("Advent of Code: Failed to parse input.")))
			.collect::<Vec<_>>();

		extrapolate(nums)
	}).sum()
}