super::get_input!(2023, 4);

super::create_solver!(
    PART_1,
    total_points,
    "Advent of Code 2023, Day 4, Part 1. Scratchcards."
);

super::create_solver!(
    PART_2,
    total_scratchcards,
    "Advent of Code 2023, Day 4, Part 2. Scratchcards."
);

fn winning_num_count(card: &str) -> Option<u32> {
    let (_, numbers) = card.split_once(':')?;
    let (winners, numbers) = numbers.split_once('|')?;
    let mut winning = [false; 100];
    for num in winners.split_whitespace().map(|n| n.parse::<usize>()) {
        winning[num.ok()?] = true;
    }
    Some(
        numbers
            .split_whitespace()
            .filter_map(|n| n.parse::<usize>().ok())
            .filter(|&n| winning[n])
            .count() as u32,
    )
}

pub fn total_points() -> u32 {
    let input = get_input();

    input
        .lines()
        .filter_map(winning_num_count)
        .filter(|&count| count > 0)
        .map(|c| 1 << (c - 1))
        .sum::<u32>()
}

pub fn total_scratchcards() -> i32 {
    let input = get_input();

    let lines = input.lines().collect::<Vec<_>>();
    let mut cards = vec![1; lines.len()];
    let mut total = 0;
    for (i, line) in lines.into_iter().enumerate() {
        let card_copies = cards[i];
        total += card_copies;
        let winners = winning_num_count(line).unwrap_or(0) as usize;
        for j in 0..winners {
            if let Some(card) = cards.get_mut(i + j + 1) {
                *card += card_copies;
            }
        }
    }
    total
}
