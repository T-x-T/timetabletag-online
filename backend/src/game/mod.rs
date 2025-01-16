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
use crate::powerup::*;

type GameId = Uuid;
type PlayerId = Uuid;

#[derive(Debug, Clone)]
pub enum Game {
	Lobby(Lobby),
	InProgress(InProgressGame),
	Finished(FinishedGame),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, Default)]
pub struct Player {
	id: Uuid,
	display_name: String,
	current_location: Location,
	timetable_cards: Vec<TimetableCard>,
	event_cards: Vec<EventCard>,
	must_use_fastest_transport_for_rounds: u8,
	luxembourg_is_germany_france_active: bool,
	lets_go_to_the_beach_active: bool,
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