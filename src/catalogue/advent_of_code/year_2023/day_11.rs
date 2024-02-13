super::get_input!(2023, 11);

super::create_solver!(PART_1, default_expansion, "Advent of Code 2023, Day 10, Part 1. Cosmic Expansion.");

super::create_solver!(PART_2, old_expansion, "Advent of Code 2023, Day 10, Part 2. Cosmic Expansion.");

fn default_expansion() -> u64 {
	calculate_galaxy_distances(2)
}

fn old_expansion() -> u64 {
	calculate_galaxy_distances(1000000)
}

fn calculate_galaxy_distances(expansion: u64) -> u64 {
	let input = get_input();

	let expansion = expansion.saturating_sub(1);

	let mut row_galaxy_counts = Vec::new();
	let mut column_galaxy_counts = Vec::new();

	input.lines().for_each(|line| {

		if column_galaxy_counts.is_empty() {
			column_galaxy_counts.resize(line.len(), 0)
		}

		let mut row_count = 0;

		for (col, c) in line.chars().enumerate() {
			if c == '#' {
				column_galaxy_counts[col] += 1;
				row_count += 1;
			}
		}

		row_galaxy_counts.push(row_count);
	});

	calculate_linear_distances(row_galaxy_counts, expansion) + 
	calculate_linear_distances(column_galaxy_counts, expansion)
}

fn calculate_linear_distances(counts: Vec<u64>, expansion: u64) -> u64 {
	let mut total_distance = 0;
    let mut sum_of_previous = 0;
    let mut expansion_offset = 0;
    let mut galaxy_index = 0;
    for (i, c) in counts.into_iter().enumerate() {
        let expanded_index = i as u64 + expansion_offset;
        for _ in 0..c {
            total_distance += galaxy_index * expanded_index - sum_of_previous;
            sum_of_previous += expanded_index;
            galaxy_index += 1;
        }
        if c == 0 {
            expansion_offset += expansion;
        }
	}

	total_distance
}