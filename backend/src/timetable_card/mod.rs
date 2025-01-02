use std::fmt::Display;
use rand::prelude::*;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
pub enum TimetableCard {
	LowSpeed,
	HighSpeed,
	Plane,
	Joker,
}

impl Display for TimetableCard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TimetableCard::LowSpeed => write!(f, "low_speed"),
			TimetableCard::HighSpeed => write!(f, "high_speed"),
			TimetableCard::Plane => write!(f, "plane"),
			TimetableCard::Joker => write!(f, "joker"),
		}
	}
}

impl std::convert::From<String> for TimetableCard {
	fn from(value: String) -> Self {
		match value.as_str() {
			"low_speed" => TimetableCard::LowSpeed,
			"high_speed" => TimetableCard::HighSpeed,
			"plane" => TimetableCard::Plane,
			"joker" => TimetableCard::Joker,
			_ => panic!("{value} not a valid TimetableCard ID"),
		}
	}
}

// There are the following number of cards in the real game:
// low_speed:  50 = 50%
// high_speed: 30 = 30%
// plane:      16 = 16%
// joker:       4 =  4%
// total:     100 =100%
pub fn generate_timetable_card_stack() -> Vec<TimetableCard> {
	let mut rng = thread_rng();
	let mut output: Vec<TimetableCard> = Vec::new();
	
	for _ in 0..50 {
		output.push(TimetableCard::LowSpeed);
	}
	for _ in 0..30 {
		output.push(TimetableCard::HighSpeed);
	}
	for _ in 0..16 {
		output.push(TimetableCard::Plane);
	}
	for _ in 0..4 {
		output.push(TimetableCard::Joker);
	}

	output.shuffle(&mut rng);

	return output;
}