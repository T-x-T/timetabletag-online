use super::*;

#[derive(Debug, Clone)]
pub struct InProgressGame {
	pub id: GameId,
	pub host: PlayerId,
	pub runner: PlayerId,
	pub players: Vec<Player>,
	pub destination: Location,
	pub current_turn: PlayerId,
	pub coins_runner: usize,
	pub coins_chasers: usize,
	pub last_used_timetable_card: Option<TimetableCard>,
	pub dice_result: Option<u8>,
	pub event_card_bought: bool,
	pub runner_path: Vec<Location>,
	pub in_progress_move: Option<InProgressMove>,
	pub timetable_card_stack: Vec<TimetableCard>,
	pub event_card_stack: Vec<EventCard>,
}

impl InProgressGame {
	pub fn make_move(&mut self, mut move_made: Move) -> Result<MoveResult, Box<dyn Error>> {
		if move_made.player_id != self.current_turn {
			return Err(Box::new(crate::CustomError::NotYourTurn));
		}

		let mut move_result = MoveResult::default();

		let player: Player = self.players.clone().into_iter().find(|x| x.id == move_made.player_id).unwrap();

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

			if !self.players.iter().find(|x| x.id == player.id).unwrap().timetable_cards.contains(&move_made.use_timetable_card_parsed.clone().unwrap()) {
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

			//Check if player wants to move to space occupied by a chaser
			if self.players.iter()
				.filter(|x| x.id != self.runner)
				.filter(|x| x.current_location == move_made.next_location_parsed.unwrap())
				.count() > 0 {
					return Err(Box::new(crate::CustomError::InvalidNextLocation));
				}

			//Remove used timetable card from player
			let mut already_removed = false;
			self.players = self.players.clone().into_iter().map(|mut x| {
				if x.id != move_made.player_id{
					return x;
				}
				x.timetable_cards.retain(|x| if x != move_made.use_timetable_card_parsed.as_ref().unwrap() || already_removed {
					true
				} else {
					already_removed = true;
					false
				});
				return x;
			}).collect();

			if self.players.iter().find(|x| x.id == player.id).unwrap().timetable_cards.is_empty() {
				move_result.finished_game = Some(FinishedGame {
					id: self.id,
					host: self.host,
					runner: self.runner,
					players: self.players.clone(),
					destination: self.destination,
					coins_runner: self.coins_runner,
					coins_chasers: self.coins_chasers,
					winning_team: Team::Chaser,
					win_condition: WinCondition::TimetableCardsRanOut,
					runner_path: self.runner_path.clone(), 
				});

				return Ok(move_result);
			}

			self.last_used_timetable_card = move_made.use_timetable_card_parsed;

			if self.runner == player.id {
				self.runner_path.push(move_made.next_location_parsed.unwrap());

				if move_made.next_location_parsed.unwrap() == self.destination && self.coins_runner >= 10 {

					move_result.finished_game = Some(FinishedGame {
						id: self.id,
						host: self.host,
						runner: self.runner,
						players: self.players.clone(),
						destination: self.destination,
						coins_runner: self.coins_runner,
						coins_chasers: self.coins_chasers,
						winning_team: Team::Runner,
						win_condition: WinCondition::GotToDestination,
						runner_path: self.runner_path.clone(), 
					});

					return Ok(move_result);
				}
			}

			let runner_location = self.players.iter().filter(|x| x.id == self.runner).next().unwrap().current_location;
			if move_made.next_location_parsed.unwrap() == runner_location {

				move_result.finished_game = Some(FinishedGame {
					id: self.id,
					host: self.host,
					runner: self.runner,
					players: self.players.clone(),
					destination: self.destination,
					coins_runner: self.coins_runner,
					coins_chasers: self.coins_chasers,
					winning_team: Team::Chaser,
					win_condition: WinCondition::RunnerCaught,
					runner_path: self.runner_path.clone(), 
				});

				move_result.runner_caught = true;

				return Ok(move_result);
			}

			if move_made.next_location_parsed.unwrap().is_coin_field() {
				let mut rng = thread_rng();
				let coins = rng.gen_range(1..=6);

				move_result.coins_received = Some(coins);

				if self.current_turn == self.runner {
					self.coins_runner += coins;
				} else {
					self.coins_chasers += coins;
				}
			}

			if !self.timetable_card_stack.is_empty() {
				let timetable_card = self.timetable_card_stack.pop().unwrap();
				move_result.timetable_cards_received = vec![timetable_card.clone()];
				
				self.players = self.players.clone().into_iter().map(|mut x| {
					if x.id == player.id {
						x.timetable_cards.push(timetable_card.clone());
					}
					return x;
				}).collect();
			}

			self.players = self.players.clone().into_iter().map(|x| {
				if x.id != player.id {
					return x;
				}
				return Player {current_location: move_made.next_location_parsed.unwrap(), ..x };
			}).collect();

			self.in_progress_move.as_mut().unwrap().new_location_already_sent = true;
			self.in_progress_move.as_mut().unwrap().use_timetable_card_already_sent = true;
		}

		

		//TODO: buy powerups
		//TODO: buy event card when landing on event spot
		//TODO: use event card
		//TODO: event card effects?
		//TODO: throwing up to two timetable cards away

		if move_made.finish_move && !self.in_progress_move.as_ref().unwrap().new_location_already_sent {
			return Err(Box::new(crate::CustomError::ActionNotAllowed));
		}
		
		if move_made.finish_move {
			self.in_progress_move = None;

			//Write next player into self.current_turn
			if self.runner_path.len() != 1 {
				let current_players_position = self.players.iter().position(|x| x.id == move_made.player_id).unwrap();
				if current_players_position == self.players.len() - 1 {
					self.current_turn = self.players.first().unwrap().id;
				} else {
					self.current_turn = self.players.iter().nth(current_players_position + 1).unwrap().id;
				}
			}
		}

		//TODO: actually send move result
		return Ok(move_result);
	}
}



#[derive(Debug, Clone, serde::Deserialize, Default)]
pub struct Move {
	pub player_id: PlayerId,
	pub next_location: Option<String>,
	pub next_location_parsed: Option<Location>,
	pub use_timetable_card: Option<String>,
	pub use_timetable_card_parsed: Option<TimetableCard>,
	pub buy_event_card: bool,
	pub use_event_card: Option<String>,
	pub buy_powerup: Option<String>,
	pub throw_timetable_cards_away: Vec<String>,
	pub finish_move: bool,
}

#[derive(Debug, Clone, Default)]
pub struct InProgressMove {
	pub move_data: Move,
	pub new_location_already_sent: bool,
	pub use_timetable_card_already_sent: bool,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct MoveResult {
	pub coins_received: Option<usize>,
	pub event_card_received: Option<EventCard>,
	pub event_card_bought: bool,
	pub runner_caught: bool,
	pub timetable_cards_received: Vec<TimetableCard>,
	pub finished_game: Option<FinishedGame>,
}
