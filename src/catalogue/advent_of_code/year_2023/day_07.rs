super::get_input!(2023, 7);

super::create_solver!(PART_1, total_win, "Advent of Code 2023, Day 7, Part 1. Camel Cards");
super::create_solver!(PART_2, total_joker, "Advent of Code 2023, Day 7, Part 2. Camel Cards");

struct ParseError;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Card {
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	Ten,
	Jack,
	Queen,
	King,
	Ace,
}

impl Default for Card {
	fn default() -> Self {
		Self::Two
	}
}

impl TryFrom<char> for Card {
	type Error = ParseError;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'2' => Self::Two,
			'3' => Self::Three,
			'4' => Self::Four,
			'5' => Self::Five,
			'6' => Self::Six,
			'7' => Self::Seven,
			'8' => Self::Eight,
			'9' => Self::Nine,
			'T' => Self::Ten,
			'J' => Self::Jack,
			'Q' => Self::Queen,
			'K' => Self::King,
			'A' => Self::Ace,
			_ => return Err(ParseError),
		})
	}
}

impl Card {
	fn value(&self) -> u8 {
		match self {
			Card::Two => 2,
			Card::Three => 3,
			Card::Four => 4,
			Card::Five => 5,
			Card::Six => 6,
			Card::Seven => 7,
			Card::Eight => 8,
			Card::Nine => 9,
			Card::Ten => 10,
			Card::Jack => 11,
			Card::Queen => 12,
			Card::King => 13,
			Card::Ace => 14
		}
	}
}

#[derive(PartialEq,	Eq, Clone, Copy, Debug)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeOfAKind,
	FullHouse,
	FourOfAKind,
	FiveOfAKind,
}

impl PartialOrd for HandType {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for HandType {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match self {
			Self::HighCard => match other {
				Self::HighCard => std::cmp::Ordering::Equal,
				_ => std::cmp::Ordering::Less,
			},
			Self::OnePair => match other {
				Self::HighCard => std::cmp::Ordering::Greater,
				Self::OnePair => std::cmp::Ordering::Equal,
				_ => std::cmp::Ordering::Less,
			},
			Self::TwoPair => match other {
				Self::HighCard | Self::OnePair => std::cmp::Ordering::Greater,
				Self::TwoPair => std::cmp::Ordering::Equal,
				_ => std::cmp::Ordering::Less,
			},
			Self::ThreeOfAKind => match other {
				Self::HighCard | Self::OnePair | Self::TwoPair => std::cmp::Ordering::Greater,
				Self::ThreeOfAKind => std::cmp::Ordering::Equal,
				_ => std::cmp::Ordering::Less,
			},
			Self::FullHouse => match other {
				Self::HighCard | Self::OnePair | Self::TwoPair | Self::ThreeOfAKind => std::cmp::Ordering::Greater,
				Self::FullHouse => std::cmp::Ordering::Equal,
				_ => std::cmp::Ordering::Less,
			},
			Self::FourOfAKind => match other {
				Self::HighCard | Self::OnePair | Self::TwoPair | Self::ThreeOfAKind | Self::FullHouse => std::cmp::Ordering::Greater,
				Self::FourOfAKind => std::cmp::Ordering::Equal,
				_ => std::cmp::Ordering::Less,
			},
			Self::FiveOfAKind => match other {
				Self::FiveOfAKind => std::cmp::Ordering::Equal,
				_ => std::cmp::Ordering::Greater,
			},
		}
	}
}


impl Default for HandType {
	fn default() -> Self {
		Self::HighCard
	}
}

impl HandType {
	fn from(cards: [Card; 5], joker: Option<Card>) -> Self {
		let mut counts = [0; 13];
		let mut jokers = 0;

		for card in cards.iter() {
			if joker.filter(|c| c == card).is_some() {
				jokers += 1;
				continue;
			}
			counts[card.value() as usize - 2] += 1;
		}

		let counts = counts.iter().filter(|&&count| count != 0);
		
		let mut fives = 0;
		let mut fours = 0;
		let mut threes = 0;
		let mut twos = 0;
		for count in counts {
			match count {
				5 => fives += 1,
				4 => fours += 1,
				3 => threes += 1,
				2 => twos += 1,
				_ => (),
			}
		}

		match (fives, fours, threes, twos, jokers) {
			(1, _, _, _, _) => HandType::FiveOfAKind,
			(_, 1, _, _, 1) => HandType::FiveOfAKind,
			(_, _, 1, _, 2) => HandType::FiveOfAKind,
			(_, _, _, 1, 3) => HandType::FiveOfAKind,
			(_, _, _, _, 4) => HandType::FiveOfAKind,
			(_, _, _, _, 5) => HandType::FiveOfAKind,
			(_, 1, _, _, _) => HandType::FourOfAKind,
			(_, _, 1, _, 1) => HandType::FourOfAKind,
			(_, _, 0, 1, 2) => HandType::FourOfAKind,
			(_, _, 0, 0, 3) => HandType::FourOfAKind,
			(_, _, 1, 1, _) => HandType::FullHouse,
			(_, _, 0, 2, 1) => HandType::FullHouse,
			(_, _, 1, 0, _) => HandType::ThreeOfAKind,
			(_, _, 0, 1, 1) => HandType::ThreeOfAKind,
			(_, _, 0, 0, 2) => HandType::ThreeOfAKind,
			(_, _, _, 2, _) => HandType::TwoPair,
			(_, _, _, 1, _) => HandType::OnePair,
			(0, 0, 0, 0, 1) => HandType::OnePair,
			_ => HandType::HighCard,
		}
	}
}

#[derive(PartialEq, Eq, Default, Clone, Copy)]
struct Hand {
	hand_type: HandType,
	joker: Option<Card>,
	cards: [u8; 5],
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}	
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match self.hand_type.cmp(&other.hand_type) {
			std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
			ordering => ordering,
		}
	}
}

impl Hand {
	fn from_str(s: &str, joker: Option<Card>) -> Result<Self, ParseError> {
		if s.len() != 5 {
			return Err(ParseError);
		}

		let mut hand = Self::default();
		let mut cards: [Card; 5] = Default::default();

		for (i, card) in s.chars().map(Card::try_from).enumerate() {
			let card = card?;
			cards[i] = card;
			hand.cards[i] = match (joker, cards[i]) {
				(Some(j), c) if c == j => 0,
				(Some(j), _) => cards[i].value() + 1,
				(None, _) => cards[i].value(),
			}
		}

		let hand_type = HandType::from(cards, joker);

		Ok(Self {
			hand_type,
			joker,
			cards: hand.cards,
		})
	}
}

fn total_win() -> u64 {
	calculate(None)
}

fn total_joker() -> u64 {
	calculate(Some(Card::Jack))
}

fn calculate(joker: Option<Card>) -> u64 {
	let input = get_input();

	let mut bids = input.lines()
		.map(|line| {
			let (hand_str, bid_str) = line.split_once(' ').ok_or(ParseError)?;
			let hand = Hand::from_str(hand_str, joker)?;
			let bid = bid_str.parse::<u64>().map_err(|_| ParseError)?;
			Ok::<(Hand, u64), ParseError>((hand, bid))
		})
		.collect::<Result<Vec<_>, _>>()
		.unwrap_or_else(|_| panic!("Failed to parse input"));

	bids.sort_unstable_by_key(|hand| hand.0);

	bids.iter().enumerate().map(|(i, (_, bid))| {
		bid * (i as u64 + 1)
	}).sum()
}