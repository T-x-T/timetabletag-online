#[cfg(test)]
mod test;

pub mod rest_api;
pub mod lobby;
pub mod in_progress_game;
pub mod finished_game;

use uuid::Uuid;
use std::collections::BTreeMap;
use std::error::Error;
use rand::prelude::*;
use crate::timetable_card::*;
use crate::event_card::*;
use crate::location::*;
use lobby::Lobby;
use in_progress_game::InProgressGame;
use finished_game::FinishedGame;

type GameId = Uuid;
type PlayerId = Uuid;

#[derive(Debug, Clone)]
pub enum Game {
	Lobby(Lobby),
	InProgress(InProgressGame),
	Finished(FinishedGame),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct Player {
	id: Uuid,
	display_name: String,
	current_location: Location,
}

#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct Move {
	player_id: PlayerId,
	next_location: Option<String>,
	next_location_parsed: Option<Location>,
	use_timetable_card: Option<String>,
	use_timetable_card_parsed: Option<TimetableCard>,
	buy_event_card: bool,
	use_event_card: Option<String>,
	buy_powerup: Option<String>,
	throw_timetable_cards_away: Vec<String>,
	finish_move: bool,
}

#[derive(Debug, Clone, Default)]
pub struct InProgressMove {
	move_data: Move,
	new_location_already_sent: bool,
	use_timetable_card_already_sent: bool,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct MoveResult {
	coins_received: Option<usize>,
	event_card_received: Option<EventCard>,
	event_card_bought: bool,
	runner_caught: bool,
	timetable_cards_received: Vec<TimetableCard>,
	finished_game: Option<FinishedGame>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum Team {
	Runner,
	Chaser,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum WinCondition {
	RunnerCaught,
	GotToDestination,
	TimetableCardsRanOut,
}