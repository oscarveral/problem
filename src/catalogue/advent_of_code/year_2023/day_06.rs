use std::str::FromStr;

super::get_input!(2023, 6);

super::create_solver!(
    PART_1,
    number_of_ways,
    "Advent of Code 2023, Day 6, Part 1. Wait For It."
);

super::create_solver!(
    PART_2,
    single_race,
    "Advent of Code 2023, Day 6, Part 2. Wait For It."
);

struct Competition {
    races: [Race; 4],
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn number_of_records(&self) -> u64 {
        // Optimization problem. Need the two points where:
        // best_distance = distance
        // best_distance = v * race_time
        // best_distance = v * (time - charge_time)
        // As v = charge_time
        // best_distance = charge_time * (time - charge_time)
        // best_distance = charge_time * time - charge_time^2
        // charge_time^2 - charge_time * time + best_distance = 0
        // charge_time = (time +- sqrt(time^2 - 4 * best_distance)) * 0.5
        // This outputs the bounds of the charge time. Between these bounds, the
        // charge time will be the optimal distance.

        let time = self.time as f64;
        let distance = self.distance as f64;
        let charge_time_high = (time + (time * time - 4.0 * distance).sqrt()) * 0.5;
        let charge_time_low = (time - (time * time - 4.0 * distance).sqrt()) * 0.5;

        // Now we need to aproximate the charge time high to the floor integer
        // and the charge time low to the ceil integer.

        let charge_time_high = charge_time_high.floor() as u64;
        let charge_time_low = charge_time_low.ceil() as u64;

        // Sum 1 because the charge time is inclusive.
        charge_time_high - charge_time_low + 1
    }
}

struct ParseError;

impl FromStr for Competition {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (times, distances) = s.split_once('\n').ok_or(ParseError)?;
        let times = times
            .split_once(':')
            .ok_or(ParseError)?
            .1
            .split_whitespace()
            .map(|time| time.parse::<u64>().map_err(|_| ParseError));
        let distances = distances
            .split_once(':')
            .ok_or(ParseError)?
            .1
            .split_whitespace()
            .map(|distance| distance.parse::<u64>().map_err(|_| ParseError));
        Ok(Self {
            races: times
                .zip(distances)
                .map(|(time, distance)| {
                    Ok(Race {
                        time: time?,
                        distance: distance?,
                    })
                })
                .collect::<Result<Vec<_>, ParseError>>()?
                .try_into()
                .map_err(|_| ParseError)?,
        })
    }
}

pub fn number_of_ways() -> u64 {
    let input = get_input();

    let competition = input
        .parse::<Competition>()
        .unwrap_or_else(|_| panic!("Failed to parse input"));

    competition
        .races
        .iter()
        .map(Race::number_of_records)
        .product::<u64>()
}

pub fn single_race() -> u64 {
    let input = get_input();

    let (times, distances) = input
        .split_once('\n')
        .unwrap_or_else(|| panic!("Failed to parse input"));

    let time = times
        .split_once(':')
        .unwrap_or_else(|| panic!("Failed to parse input"))
        .1
        .chars()
        .filter_map(|c| c.to_digit(10).map(u64::from))
        .reduce(|acc, b| acc * 10 + b)
        .unwrap_or_else(|| panic!("Failed to parse input"));

    let distance = distances
        .split_once(':')
        .unwrap_or_else(|| panic!("Failed to parse input"))
        .1
        .chars()
        .filter_map(|c| c.to_digit(10).map(u64::from))
        .reduce(|acc, b| acc * 10 + b)
        .unwrap_or_else(|| panic!("Failed to parse input"));

    let race = Race { time, distance };

    race.number_of_records()
}
