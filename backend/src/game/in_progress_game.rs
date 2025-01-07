use super::*;

#[derive(Debug, Clone)]
pub struct InProgressGame {
	pub id: GameId,
	pub host: PlayerId,
	pub runner: Player, //replace with just the players id?
	pub players: Vec<Player>,
	pub destination: Location,
	pub current_turn: Option<Player>,
	pub coins_runner: usize,
	pub coins_chasers: usize,
	pub timetable_cards: BTreeMap<PlayerId, Vec<TimetableCard>>, //integrate into Player?
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
		if !self.current_turn.as_ref().is_some_and(|x| x.id == move_made.player_id) {
			return Err(Box::new(crate::CustomError::NotYourTurn));
		}

		let mut move_result = MoveResult::default();

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

			if self.players.iter()
				.filter(|x| x.id != self.runner.id)
				.filter(|x| x.current_location == move_made.next_location_parsed.unwrap())
				.count() > 0 {
					return Err(Box::new(crate::CustomError::InvalidNextLocation));
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

			if self.timetable_cards.get(&player.id).unwrap().is_empty() {
				move_result.finished_game = Some(FinishedGame {
					id: self.id,
					host: self.host,
					runner: self.runner.id,
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

			if self.runner.id == player.id {
				self.runner_path.push(move_made.next_location_parsed.unwrap());

				if move_made.next_location_parsed.unwrap() == self.destination && self.coins_runner >= 10 {

					move_result.finished_game = Some(FinishedGame {
						id: self.id,
						host: self.host,
						runner: self.runner.id,
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

			if move_made.next_location_parsed.unwrap() == self.runner.current_location {

				move_result.finished_game = Some(FinishedGame {
					id: self.id,
					host: self.host,
					runner: self.runner.id,
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

				if self.current_turn.as_ref().unwrap().id == self.runner.id {
					self.coins_runner += coins;
				} else {
					self.coins_chasers += coins;
				}
			}

			if !self.timetable_card_stack.is_empty() {
				let timetable_card = self.timetable_card_stack.pop().unwrap();
				move_result.timetable_cards_received = vec![timetable_card.clone()];
				self.timetable_cards.entry(player.id).and_modify(|x| x.push(timetable_card));
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
					self.current_turn = self.players.first().cloned();
				} else {
					self.current_turn = self.players.iter().nth(current_players_position + 1).cloned();
				}
			}
		}

		//TODO: actually send move result
		return Ok(move_result);
	}
}
