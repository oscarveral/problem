use std::str::FromStr;

super::get_input!(2023, 2);

super::create_solver!(
    PART_1,
    sum_of_ids,
    "Advent of Code 2023, Day 1, Part 1. Cube Conundrum."
);
super::create_solver!(
    PART_2,
    sum_power_sets,
    "Advent of Code 2023, Day 1, Part 2. Cube Conundrum."
);

struct ParseError;

#[derive(Clone, Copy)]
struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = Color {
            red: 0,
            green: 0,
            blue: 0,
        };

        for col in s.split(',') {
            let (num_str, color) = col.trim().split_once(' ').ok_or(ParseError)?;
            let num = num_str.parse::<u32>().map_err(|_| ParseError)?;
            match color {
                "red" => res.red += num,
                "green" => res.green += num,
                "blue" => res.blue += num,
                _ => return Err(ParseError),
            }
        }
        Ok(res)
    }
}

impl Color {
    fn is_posible(&self, limit: &Self) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }

    fn pow(self) -> u32 {
        self.red * self.green * self.blue
    }

    fn max(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

struct Game {
    id: u32,
    colors: [Color; 9],
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, colors) = s.split_once(':').ok_or(ParseError)?;
        if id.len() != 6 {
            return Err(ParseError);
        }

        let id = id[5..].parse::<u32>().map_err(|_| ParseError)?;
        let colors: [Color; 9] = colors
            .split(';')
            .map(|s| s.parse::<Color>())
            .collect::<Result<Vec<Color>, ParseError>>()?
            .try_into()
            .map_err(|_| ParseError)?;
        Ok(Self { id, colors })
    }
}

impl Game {
    fn is_posible(&self, limit: Color) -> bool {
        self.colors.iter().all(|color| color.is_posible(&limit))
    }
}

pub fn sum_of_ids() -> u32 {
    let input = get_input();

    input
        .lines()
        .filter_map(|line| line.parse::<Game>().ok())
        .filter(|game| {
            game.is_posible(Color {
                red: 12,
                green: 13,
                blue: 14,
            })
        })
        .map(|game| game.id)
        .sum::<u32>()
}

pub fn sum_power_sets() -> u32 {
    let input = get_input();

    input
        .lines()
        .filter_map(|line| line.parse::<Game>().ok())
        .filter_map(|game| {
            game.colors
                .iter()
                .copied()
                .reduce(|acc, color| acc.max(color))
        })
        .map(|color| color.pow())
        .sum::<u32>()
}
