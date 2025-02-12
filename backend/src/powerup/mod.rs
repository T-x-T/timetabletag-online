use std::fmt::Display;
use crate::location::{Country, Location};

#[derive(Debug, Clone, serde::Serialize)]
pub enum Powerup {
	LearnRunnerCountry,
	LearnRunnerLocation,
	ChaserGetsTwoTurns,
	LearnRunnerDestination,
}

impl Display for Powerup {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Powerup::LearnRunnerCountry => write!(f, "learn_runner_country"),
			Powerup::LearnRunnerLocation => write!(f, "learn_runner_location"),
			Powerup::ChaserGetsTwoTurns => write!(f, "chaser_gets_two_turns"),
			Powerup::LearnRunnerDestination => write!(f, "learn_runner_destination"),
		}
	}
}

impl TryFrom<&str> for Powerup {
	type Error = String;
	fn try_from(value: &str) -> Result<Powerup, String> {
		match value {
			"learn_runner_country" => Ok(Powerup::LearnRunnerCountry),
			"learn_runner_location" => Ok(Powerup::LearnRunnerLocation),
			"chaser_gets_two_turns" => Ok(Powerup::ChaserGetsTwoTurns),
			"learn_runner_destination" => Ok(Powerup::LearnRunnerDestination),
			_ => Err(format!("{value} is not a valid powerup")),
		}
	}
}

impl Powerup {
	pub fn get_price(&self, chaser_count: usize) -> usize {
		return match self {
			Powerup::LearnRunnerCountry => if chaser_count == 2 { 5 } else { 10 },
			Powerup::LearnRunnerLocation => if chaser_count == 2 { 10 } else { 20 },
			Powerup::ChaserGetsTwoTurns => if chaser_count == 2 { 15 } else { 30 },
			Powerup::LearnRunnerDestination => if chaser_count == 2 { 20 } else { 40 },
		}
	}
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct PowerupStatus {
	pub runner_country: Option<Country>,
	pub runner_location: Option<Location>,
	pub runner_destination: Option<Location>,
	pub get_another_turn: bool,
}