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
	pub power_up_status: PowerupStatus,
	pub get_another_turn: bool,
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

			if player_wants_to_move_space_occupied_by_chaser(&self.players, self.runner, move_made.next_location_parsed.unwrap()) {
				return Err(Box::new(crate::CustomError::InvalidNextLocation));
			}

			self.players = remove_used_timetable_card_from_player(self.players.clone(), move_made.player_id, move_made.use_timetable_card_parsed.as_ref().unwrap());

			if self.players.iter().find(|x| x.id == player.id).unwrap().timetable_cards.is_empty() {
				move_result.finished_game = Some(FinishedGame::from_in_progress_game(&self, Team::Chaser, WinCondition::TimetableCardsRanOut));
				return Ok(move_result);
			}

			self.last_used_timetable_card = move_made.use_timetable_card_parsed;

			if self.runner == player.id {
				self.runner_path.push(move_made.next_location_parsed.unwrap());

				if move_made.next_location_parsed.unwrap() == self.destination && self.coins_runner >= 10 {
					move_result.finished_game = Some(FinishedGame::from_in_progress_game(&self, Team::Runner, WinCondition::GotToDestination));
					return Ok(move_result);
				}
			}

			let runner_location = self.players.iter().filter(|x| x.id == self.runner).next().unwrap().current_location;
			if move_made.next_location_parsed.unwrap() == runner_location {
				move_result.finished_game = Some(FinishedGame::from_in_progress_game(&self, Team::Chaser, WinCondition::RunnerCaught));
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

		
		if move_made.buy_powerup.is_some() && player.id != self.runner {
			let powerup: Powerup = move_made.buy_powerup.unwrap().as_str().into();

			if self.coins_chasers < powerup.get_price(self.players.len() - 1) {
				return Err(Box::new(crate::CustomError::NotEnoughCoins));
			}

			self.coins_chasers -= powerup.get_price(self.players.len() - 1);

			match powerup {
				Powerup::LearnRunnerCountry => {
					move_result.power_up_status.runner_country = Some(self.players.iter().find(|x| x.id == self.runner).unwrap().current_location.into());
				},
				Powerup::LearnRunnerLocation => {
					move_result.power_up_status.runner_location = Some(self.players.iter().find(|x| x.id == self.runner).unwrap().current_location);
				},
				Powerup::ChaserGetsTwoTurns => {
					move_result.power_up_status.get_another_turn = true;
					self.get_another_turn = true;
				},
				Powerup::LearnRunnerDestination => {
					move_result.power_up_status.runner_destination = Some(self.destination);
				},
			};

			self.power_up_status = move_result.power_up_status.clone(); //TODO: return this to all players in get game_state api call
		}

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
			if !self.get_another_turn {
				let current_players_position = self.players.iter().position(|x| x.id == move_made.player_id).unwrap();
				if current_players_position == self.players.len() - 1 {
					self.current_turn = self.players.first().unwrap().id;
				} else {
					self.current_turn = self.players.iter().nth(current_players_position + 1).unwrap().id;
				}
			}
			self.get_another_turn = false;
		}

		//TODO: actually send move result
		return Ok(move_result);
	}
}

fn player_wants_to_move_space_occupied_by_chaser(players: &Vec<Player>, runner: PlayerId, next_location: Location) -> bool {
	return players.iter()
		.filter(|x| x.id != runner)
		.filter(|x| x.current_location == next_location)
		.count() > 0;
}

fn remove_used_timetable_card_from_player(players: Vec<Player>, player_id: PlayerId, timetable_card_used: &TimetableCard) -> Vec<Player> {
	let mut already_removed = false;
	return players.into_iter().map(|mut x| {
		if x.id != player_id{
			return x;
		}
		x.timetable_cards.retain(|x| if x != timetable_card_used || already_removed {
			true
		} else {
			already_removed = true;
			false
		});
		return x;
	}).collect();
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
	pub power_up_status: PowerupStatus,
}