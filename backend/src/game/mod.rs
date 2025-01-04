#[cfg(test)]
mod test;

pub mod rest_api;

use uuid::Uuid;
use std::collections::BTreeMap;
use std::error::Error;
use rand::prelude::*;
use crate::timetable_card::*;
use crate::event_card::*;
use crate::location::*;

type GameId = Uuid;
type PlayerId = Uuid;

#[derive(Debug, Clone)]
pub struct Game {
	id: GameId,
	invite_code: String,
	host: PlayerId,
	state: GameState,
	runner: Option<Player>,
	players: Vec<Player>,
	destination: String,
	current_turn: Option<Player>,
	coins_runner: usize,
	coins_chasers: usize,
	timetable_cards: BTreeMap<PlayerId, Vec<TimetableCard>>,
	last_used_timetable_card: Option<TimetableCard>,
	dice_result: Option<u8>,
	event_card_bought: bool,
	winning_team: Option<String>,
	win_condition: Option<String>,
	runner_path: Vec<Location>,
	in_progress_move: Option<InProgressMove>,
	timetable_card_stack: Vec<TimetableCard>,
	event_card_stack: Vec<EventCard>,
}

impl Game {
	pub fn create(display_name: String) -> Self {
		let mut rng = thread_rng();
		let rand_destination_index = rng.gen_range(0..=4);
		let invite_code_part1 = rng.gen_range(0..=999);
		let invite_code_part2 = rng.gen_range(0..=999);

		let player_id = PlayerId::new_v4();
		let player = Player {
			id: player_id.clone(),
			display_name,
			current_location: Location::Nancy,
		};

		return Self {
			id: GameId::new_v4(),
			invite_code: format!("{invite_code_part1:0>3}-{invite_code_part2:0>3}"), //TODO: collision possible
			host: player_id,
			state: GameState::Lobby,
			runner: None,
			players: vec![player],
			destination: ["dublin", "copenhagen", "vienna", "rome", "madrid"].into_iter().nth(rand_destination_index).unwrap().into(),
			current_turn: None,
			coins_runner: 0,
			coins_chasers: 0,
			timetable_cards: BTreeMap::new(),
			last_used_timetable_card: None,
			dice_result: None,
			event_card_bought: false,
			winning_team: None,
			win_condition: None,
			runner_path: Vec::new(),
			in_progress_move: None,
			timetable_card_stack: Vec::new(),
			event_card_stack: Vec::new(),
		}
	}

	pub fn join(&mut self, display_name: String) -> Result<PlayerId, Box<dyn Error>> {
		if self.players.len() >= 4 {
			return Err(Box::new(crate::CustomError::LobbyFull));
		}
		
		let id = PlayerId::new_v4();
		let player = Player {
			id: id.clone(),
			display_name,
			current_location: Location::Nancy,
		};

		self.players.push(player);
		
		return Ok(id);
	}

	pub fn start(&mut self, player_id: PlayerId) -> Result<(), Box<dyn Error>> {
		if player_id != self.host {
			return Err(Box::new(crate::CustomError::ActionNotAllowed));
		}
		
		if self.players.len() <= 2 {
			return Err(Box::new(crate::CustomError::LobbyNotFullEnough));
		}

		if self.state != GameState::Lobby {
			return Err(Box::new(crate::CustomError::InvalidGameState));
		}
		
		let mut rng = thread_rng();
		let rand_player_id = rng.gen_range(0..=self.players.len() - 1);
		
		self.runner = Some(self.players.iter().nth(rand_player_id).unwrap().clone());
		self.current_turn = Some(self.runner.clone().unwrap());
		
		self.timetable_card_stack = generate_timetable_card_stack();

		self.players.iter().for_each(|player| {
			self.timetable_cards.insert(player.id, vec![self.timetable_card_stack.pop().unwrap(), self.timetable_card_stack.pop().unwrap(), self.timetable_card_stack.pop().unwrap(), self.timetable_card_stack.pop().unwrap(), self.timetable_card_stack.pop().unwrap()]);
		});		

		self.event_card_stack = generate_event_card_stack();

		self.state = GameState::InProgress;
		return Ok(());
	}

	pub fn make_move(&mut self, mut move_made: Move) -> Result<MoveResult, Box<dyn Error>> {
		if !self.current_turn.as_ref().is_some_and(|x| x.id == move_made.player_id) {
			return Err(Box::new(crate::CustomError::NotYourTurn));
		}

		let player: &Player = self.players.iter().filter(|x| x.id == move_made.player_id).next().unwrap();

		if move_made.next_location.is_some() {
			move_made.next_location_parsed = Some(Location::from(move_made.next_location.clone().unwrap()));
		}

		if move_made.use_timetable_card.is_some() {
			move_made.use_timetable_card_parsed = Some(TimetableCard::from(move_made.use_timetable_card.clone().unwrap()))
		}
		
		if self.in_progress_move.is_none() {
			self.in_progress_move = Some(InProgressMove {
				move_data: move_made.clone(),
				new_location_already_sent: false,
				use_timetable_card_already_sent: false,
			});
		}

		if move_made.next_location_parsed.is_some() && move_made.use_timetable_card_parsed.is_some() {
			if self.in_progress_move.as_ref().unwrap().new_location_already_sent {
				return Err(Box::new(crate::CustomError::AlreadyMoved));
			}

			if !self.timetable_cards.get(&player.id).unwrap().contains(&move_made.use_timetable_card_parsed.clone().unwrap()) {
				return Err(Box::new(crate::CustomError::MissingCard));
			}

			let current_location = &player.current_location;
			match move_made.use_timetable_card_parsed.clone().unwrap() {
				TimetableCard::LowSpeed => {
					if !current_location.get_low_speed_connections().contains(&move_made.next_location_parsed.unwrap()) {
						return Err(Box::new(crate::CustomError::InvalidNextLocation));	
					}
				},
				TimetableCard::HighSpeed => {
					if !current_location.get_high_speed_connections().contains(&move_made.next_location_parsed.unwrap()) {
						return Err(Box::new(crate::CustomError::InvalidNextLocation));	
					}
				},
				TimetableCard::Plane => {
					if !current_location.get_plane_connections().contains(&move_made.next_location_parsed.unwrap()) {
						return Err(Box::new(crate::CustomError::InvalidNextLocation));	
					}
				},
				TimetableCard::Joker => {
					if !current_location.get_joker_connections().contains(&move_made.next_location_parsed.unwrap()) {
						return Err(Box::new(crate::CustomError::InvalidNextLocation));	
					}
				},
			}

			let mut already_removed = false;
			self.timetable_cards.entry(player.id).and_modify(|x| {
				x.retain(|x| if x != move_made.use_timetable_card_parsed.as_ref().unwrap() || already_removed {
					true
				} else {
					already_removed = true;
					false
				})
			});

			self.last_used_timetable_card = move_made.use_timetable_card_parsed;

			if self.runner.as_ref().unwrap().id == player.id {
				self.runner_path.push(move_made.next_location_parsed.unwrap());
			}

			//TODO: handle empty card stack
			self.timetable_cards.entry(player.id).and_modify(|x| x.push(self.timetable_card_stack.pop().unwrap()));

			self.players = self.players.clone().into_iter().map(|x| {
				if x.id != player.id {
					return x;
				}
				return Player {current_location: move_made.next_location_parsed.unwrap(), ..x };
			}).collect();

			self.in_progress_move.as_mut().unwrap().new_location_already_sent = true;
			self.in_progress_move.as_mut().unwrap().use_timetable_card_already_sent = true;
		}


		if move_made.finish_move {
			self.in_progress_move = None;

			//Write next player into self.current_turn
			let current_players_position = self.players.iter().position(|x| x.id == move_made.player_id).unwrap();
			if current_players_position == self.players.len() - 1 {
				self.current_turn = self.players.first().cloned();
			} else {
				self.current_turn = self.players.iter().nth(current_players_position + 1).cloned();
			}
		}

		return Ok(MoveResult::default());
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
	Lobby,
	InProgress,
	Finished,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Player {
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
	use_powerup: Option<String>,
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
	event_card_received: Option<String>,
	event_card_bought: bool,
	runner_caught: bool,
	timetable_cards_received: Vec<String>,
}