use super::*;

use rand::prelude::*;

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
	pub get_extra_turns: u8,
}

impl InProgressGame {
	pub fn make_move(&mut self, mut move_made: Move) -> Result<MoveResult, Box<dyn Error>> {
		if move_made.player_id != self.current_turn {
			return Err(Box::new(crate::CustomError::NotYourTurn));
		}

		let mut rng = thread_rng();
		let mut move_result = MoveResult::default();

		let mut player: Player = self.players.clone().into_iter().find(|x| x.id == move_made.player_id).unwrap();

		if move_made.next_location.is_some() {
			move_made.next_location_parsed = Some(Location::try_from(move_made.next_location.clone().unwrap())?);
		}

		if move_made.use_timetable_card.is_some() {
			move_made.use_timetable_card_parsed = Some(TimetableCard::try_from(move_made.use_timetable_card.clone().unwrap())?)
		}
		
		
		let mut in_progress_move: InProgressMove;
		if self.in_progress_move.is_none() {
			in_progress_move = InProgressMove {
				new_location_already_sent: false,
				use_timetable_card_already_sent: false,
				event_card_bought: false,
				stealth_mode_enabled: false,
			};
		} else {
			in_progress_move = self.in_progress_move.clone().unwrap();
		}

		let mut timetable_card_stack = self.timetable_card_stack.clone();
		let mut get_extra_turns = self.get_extra_turns;
		let mut coins_chasers = self.coins_chasers;
		let mut coins_runner = self.coins_runner;
		let mut last_used_timetable_card = self.last_used_timetable_card.clone();
		let mut runner_path = self.runner_path.clone();
		let mut players = self.players.clone();
		let mut event_card_bought = self.event_card_bought;
		let mut event_card_stack = self.event_card_stack.clone();

		if !move_made.throw_timetable_cards_away.is_empty() {
			if is_move_possible(&player) {
				return Err(Box::new(crate::CustomError::ValidMovePossible));
			}

			if move_made.throw_timetable_cards_away.len() > 2 {
				return Err(Box::new(crate::CustomError::ThrewTooManyTimetableCardsAway));
			}

			let orig_player_timetable_cards = player.timetable_cards.clone();
			for timetable_card in &move_made.throw_timetable_cards_away {
				if orig_player_timetable_cards.contains(&timetable_card.as_str().try_into()?) {
					player = remove_used_timetable_card_from_player(player, &timetable_card.as_str().try_into()?);

					if !timetable_card_stack.is_empty() {
						let timetable_card = timetable_card_stack.pop().unwrap();
						move_result.timetable_cards_received.push(timetable_card.clone());
						
						player.timetable_cards.push(timetable_card);
					}

				} else {
					return Err(Box::new(crate::CustomError::MissingTimetableCard));
				}
			}

			in_progress_move.new_location_already_sent = true;
		}

		if !in_progress_move.stealth_mode_enabled {
			player.stealth_mode_active = false;
		}

		if player.lets_go_to_the_beach_active && move_made.next_location_parsed.is_some() && move_made.use_timetable_card_parsed.is_none() {
			if !player.current_location.is_coastal() {
				return Err(Box::new(crate::CustomError::InvalidNextLocation));
			}

			if !move_made.next_location_parsed.unwrap().is_coastal() {
				return Err(Box::new(crate::CustomError::InvalidNextLocation));
			}
			
			player.lets_go_to_the_beach_active = false;

			player.current_location = move_made.next_location_parsed.unwrap();
			in_progress_move.new_location_already_sent = true;
			in_progress_move.use_timetable_card_already_sent = true;
		}

		if player.next_move_must_go_north_active && player.current_location.get_north_connections().is_empty() {
			player.current_location = move_made.next_location_parsed.unwrap();
			in_progress_move.new_location_already_sent = true;
			in_progress_move.use_timetable_card_already_sent = true;

			player.next_move_must_go_north_active = false;
		}

		if player.zug_faellt_aus_active && move_made.next_location_parsed.is_some() && Country::from(player.current_location) == Country::Germany {
			in_progress_move.new_location_already_sent = true;
			in_progress_move.use_timetable_card_already_sent = true;

			player.zug_faellt_aus_active = false;
		}

		if player.slovenia_as_a_treat_active && move_made.next_location_parsed.is_some() {
			player.slovenia_as_a_treat_active = false;

			if move_made.next_location_parsed.unwrap() == Location::Ljubljana {
				in_progress_move.new_location_already_sent = true;
				in_progress_move.use_timetable_card_already_sent = true;
				player.current_location = Location::Ljubljana;
			}
		}

		if move_made.next_location_parsed.is_some() && move_made.use_timetable_card_parsed.is_some() && !in_progress_move.new_location_already_sent {
			if !players.iter().find(|x| x.id == player.id).unwrap().timetable_cards.contains(&move_made.use_timetable_card_parsed.clone().unwrap()) {
				return Err(Box::new(crate::CustomError::MissingTimetableCard));
			}

			let current_location = player.current_location;

			if player.leave_country_immediately_active {
				if player_can_leave_country(&player) {
					if Country::from(player.current_location) != Country::from(move_made.next_location_parsed.unwrap()) {
						player.leave_country_immediately_active = false;
					} else {
						return Err(Box::new(crate::CustomError::YouMustLeaveTheCountryImmediately));
					}
				} else {
					player.leave_country_immediately_active = false;
				}
			}

			if player.next_move_must_go_north_active {
				if !current_location.get_north_connections().contains(move_made.next_location_parsed.as_ref().unwrap()) {
					return Err(Box::new(crate::CustomError::YouMustGoNorth));
				} else {
					player.next_move_must_go_north_active = false;
				}
			}

			if player.must_use_fastest_transport_for_rounds > 0 {
				if !player_used_fastest_transport_method(move_made.use_timetable_card_parsed.as_ref().unwrap(), &player, current_location){
					return Err(Box::new(crate::CustomError::YoureCurrentlyHuntedByMenForSport));
				}

				player.must_use_fastest_transport_for_rounds -= 1;
			}

			if player.must_use_slowest_transport_for_rounds > 0 {
				if !player_used_slowest_transport_method(move_made.use_timetable_card_parsed.as_ref().unwrap(), &player) {
					return Err(Box::new(crate::CustomError::YouAreCurrentlyInRatMode));
				}

				player.must_use_slowest_transport_for_rounds -= 1;
			}

			if !can_get_to_next_location_with_used_timetable_card(move_made.use_timetable_card_parsed.as_ref().unwrap(), &player, &current_location, move_made.next_location_parsed.as_ref().unwrap()) {
				return Err(Box::new(crate::CustomError::InvalidNextLocation));
			}

			if player_wants_to_move_space_occupied_by_chaser(&players, self.runner, move_made.next_location_parsed.unwrap()) {
				return Err(Box::new(crate::CustomError::InvalidNextLocation));
			}

			if current_location == Location::Luxembourg && player.luxembourg_is_germany_france_active {
				if move_made.next_location_parsed.unwrap() == Location::Brussels {
					return Err(Box::new(crate::CustomError::YouMustGoToGermanyOrFrance));
				} else {
					player.luxembourg_is_germany_france_active = false;
					get_extra_turns = 1;
				}
			}

			player = remove_used_timetable_card_from_player(player, move_made.use_timetable_card_parsed.as_ref().unwrap());

			if player.timetable_cards.is_empty() {
				move_result.finished_game = Some(FinishedGame::from_in_progress_game(&self, Team::Chaser, WinCondition::TimetableCardsRanOut));
				return Ok(move_result);
			}

			last_used_timetable_card = move_made.use_timetable_card_parsed;

			if self.runner == player.id {
				runner_path.push(move_made.next_location_parsed.unwrap());

				if move_made.next_location_parsed.unwrap() == self.destination && coins_runner >= 10 {
					move_result.finished_game = Some(FinishedGame::from_in_progress_game(&self, Team::Runner, WinCondition::GotToDestination));
					return Ok(move_result);
				}
			}

			let runner_location = players.iter().filter(|x| x.id == self.runner).next().unwrap().current_location;
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
					coins_runner += coins;
				} else {
					coins_chasers += coins;
				}
			}

			if !timetable_card_stack.is_empty() {
				let timetable_card = timetable_card_stack.pop().unwrap();
				move_result.timetable_cards_received.push(timetable_card.clone());
				
				player.timetable_cards.push(timetable_card);
			}

			player.current_location = move_made.next_location_parsed.unwrap();
			in_progress_move.new_location_already_sent = true;
			in_progress_move.use_timetable_card_already_sent = true;
		}

		
		if move_made.buy_powerup.is_some() && player.id != self.runner {
			let powerup: Powerup = move_made.buy_powerup.unwrap().as_str().try_into()?;

			if coins_chasers < powerup.get_price(players.len() - 1) {
				return Err(Box::new(crate::CustomError::NotEnoughCoins));
			}

			coins_chasers -= powerup.get_price(players.len() - 1);

			match powerup {
				Powerup::LearnRunnerCountry => {
					move_result.power_up_status.runner_country = Some(players.iter().find(|x| x.id == self.runner).unwrap().current_location.into());
				},
				Powerup::LearnRunnerLocation => {
					move_result.power_up_status.runner_location = Some(players.iter().find(|x| x.id == self.runner).unwrap().current_location);
				},
				Powerup::ChaserGetsTwoTurns => {
					move_result.power_up_status.get_another_turn = true;
					get_extra_turns = 1;
				},
				Powerup::LearnRunnerDestination => {
					move_result.power_up_status.runner_destination = Some(self.destination);
				},
			};
		}

		if move_made.buy_event_card {
			if !in_progress_move.new_location_already_sent {
				return Err(Box::new(crate::CustomError::EventCardNoLocationSent));
			}

			if in_progress_move.event_card_bought {
				return Err(Box::new(crate::CustomError::EventCardAlreadyBought));
			}

			if !player.current_location.is_event_field() {
				return Err(Box::new(crate::CustomError::NotAnEventField));
			}

			if player.id == self.runner {
				if coins_runner < 1 {
					return Err(Box::new(crate::CustomError::NotEnoughCoins));
				}
			} else {
				if coins_chasers < 1 {
					return Err(Box::new(crate::CustomError::NotEnoughCoins));
				}
			}
			let mut event_card = event_card_stack.pop();

			if event_card.is_none() {
				return Err(Box::new(crate::CustomError::EventCardStackEmpty));
			}

			let mut instantly_play_event_card = false;

			match event_card.as_ref().unwrap() {
				EventCard::GiveMeYourCards => {
					let cloned_players = players.clone();
					let players_with_event_cards: Vec<&Player> = cloned_players.iter().filter(|x| !x.event_cards.is_empty()).collect();
					if !players_with_event_cards.is_empty() {
						let random_player_with_event_cards = players_with_event_cards.choose(&mut rng).unwrap();
						let random_event_card = random_player_with_event_cards.event_cards.choose(&mut rng).unwrap();
	
						players = players.clone().into_iter().map(|mut x| {
							if x.id == random_player_with_event_cards.id {
								x.event_cards.retain(|x| x != random_event_card);
							}
							return x;
						}).collect();

						event_card = Some(random_event_card.clone());
					} else {
						event_card = event_card_stack.pop();

						if event_card.is_none() {
							return Err(Box::new(crate::CustomError::EventCardStackEmpty));
						}
					}
				},

				EventCard::HuntedByMenForSport => {
					instantly_play_event_card = true;
					player.must_use_fastest_transport_for_rounds = 2;
				},
				EventCard::LuxembourgIsGermanyFrance => {
					instantly_play_event_card = true;
					player.luxembourg_is_germany_france_active = true;
				},
				EventCard::LetsGoToTheBeach => {
					instantly_play_event_card = true;
					player.lets_go_to_the_beach_active = true;
				},
				EventCard::ImagineTrains => {
					instantly_play_event_card = true;
					player.can_use_any_train_ticket = true;
				},
				EventCard::HydrateOrDiedrate => {
					instantly_play_event_card = true;
				},
				EventCard::StealthOutfit => {
					instantly_play_event_card = true;
					player.stealth_mode_active = true;
					in_progress_move.stealth_mode_enabled = true;
				},
				EventCard::CardinalDirectionsAndVibes => {
					instantly_play_event_card = true;
					player.next_move_must_go_north_active = true;
				},
				EventCard::Pizzazz => {
					instantly_play_event_card = true;
					let mut rng = thread_rng();
					let coins_for_runner = rng.gen_range(1..=6);
					let mut coins_for_chasers = 0;
					for _ in 0..players.len() - 1 {
						coins_for_chasers += rng.gen_range(1..=6);
					}

					if self.runner == player.id {
						move_result.coins_received = Some(coins_for_runner);
					} else {
						move_result.coins_received = Some(coins_for_chasers);
					}

					coins_runner += coins_for_runner;
					coins_chasers += coins_for_chasers;
				},
				EventCard::RatMode => {
					instantly_play_event_card = true;
					player.must_use_slowest_transport_for_rounds = 2;
				},
				EventCard::BingBong => {
					instantly_play_event_card = true;
					//Bing Bong
				},
				EventCard::LeaveCountryImmediately => {
					instantly_play_event_card = true;
					player.leave_country_immediately_active = true;
				},
				EventCard::ZugFaelltAus => {
					instantly_play_event_card = true;
					player.zug_faellt_aus_active = true;
				},
				EventCard::SnackZone => {
					instantly_play_event_card = true;
				},
				EventCard::ItsAllInTheTrees => {
					instantly_play_event_card = true;
					get_extra_turns = 1;
				},
				EventCard::BonjourToEveryone => {
					instantly_play_event_card = true;
				},
				EventCard::NoTalk => {
					instantly_play_event_card = true;
				},
				EventCard::SloveniaAsATreat => {
					instantly_play_event_card = true;
					player.slovenia_as_a_treat_active = true;
				},
				_ => (),
			}

			in_progress_move.event_card_bought = true;
			move_result.event_card_bought = true;
			event_card_bought = true;
			move_result.event_card_received = event_card.clone();

			if !instantly_play_event_card {
				player.event_cards.push(event_card.clone().unwrap());
			}

		}

		if move_made.use_event_card.is_some() {
			let event_card: EventCard = move_made.use_event_card.unwrap().try_into()?;

			if !player.event_cards.contains(&event_card) {
				return Err(Box::new(crate::CustomError::EventCardNotOnYourHand));
			}

			player.event_cards.retain(|x| *x != event_card);

			match event_card {
				EventCard::ConsiderVelocity => {
					get_extra_turns = 1;
				},
				EventCard::ItsPopsicle => {
					get_extra_turns = 1;
				},
				_ => (),
			}
		}
		
		if move_made.finish_move && !in_progress_move.new_location_already_sent {
			return Err(Box::new(crate::CustomError::ActionNotAllowed));
		}

		//NO RETURNING ERRORS BEYOND THIS POINT
		self.in_progress_move = Some(in_progress_move);
		self.timetable_card_stack = timetable_card_stack;
		self.get_extra_turns = get_extra_turns;
		self.coins_chasers = coins_chasers;
		self.coins_runner = coins_runner;
		self.last_used_timetable_card = last_used_timetable_card;
		self.runner_path = runner_path;
		self.power_up_status = move_result.power_up_status.clone();
		self.players = players;
		self.event_card_bought = event_card_bought;
		self.event_card_stack = event_card_stack;

		if move_made.finish_move {
			self.in_progress_move = None;

			//Write next player into self.current_turn
			if self.get_extra_turns == 0 {
				let current_players_position = self.players.iter().position(|x| x.id == move_made.player_id).unwrap();
				if current_players_position == self.players.len() - 1 {
					self.current_turn = self.players.first().unwrap().id;
				} else {
					self.current_turn = self.players.iter().nth(current_players_position + 1).unwrap().id;
				}
			} else {
				self.get_extra_turns -= 1;
			}
		}

		self.players = self.players.clone().into_iter().map(|mut x| {
			if x.id == player.id {
				x = player.clone();
			}
			return x;
		}).collect();

		return Ok(move_result);
	}
}

fn can_get_to_next_location_with_used_timetable_card(used_timetable_card: &TimetableCard, player: &Player, current_location: &Location, next_location: &Location) -> bool {
	match used_timetable_card {
		TimetableCard::LowSpeed => {
			if !current_location.get_low_speed_connections().contains(next_location) {
				if player.can_use_any_train_ticket {
					if !current_location.get_high_speed_connections().contains(next_location) {
						return false;
					}
				} else {
					return false;
				}
			}
		},
		TimetableCard::HighSpeed => {
			if !current_location.get_high_speed_connections().contains(next_location) {
				if player.can_use_any_train_ticket {
					if !current_location.get_low_speed_connections().contains(next_location) {
						return false;
					}
				} else {
					return false;	
				}
			}
		},
		TimetableCard::Plane => {
			if !current_location.get_plane_connections().contains(next_location) {
				return false;	
			}
		},
		TimetableCard::Joker => {
			if !current_location.get_joker_connections().contains(next_location) {
				return false;	
			}
		},
	}

	return true;
}

fn player_used_slowest_transport_method(used_timetable_card: &TimetableCard, player: &Player) -> bool {
	let mut player_used_slowest_transport_method = true;
	if player.timetable_cards.contains(&TimetableCard::LowSpeed) && used_timetable_card != &TimetableCard::LowSpeed {
		player_used_slowest_transport_method = false;
	}
	if !player.timetable_cards.contains(&TimetableCard::LowSpeed) && used_timetable_card != &TimetableCard::HighSpeed {
		player_used_slowest_transport_method = false;
	}
	return player_used_slowest_transport_method;
}

fn player_used_fastest_transport_method(used_timetable_card: &TimetableCard, player: &Player, current_location: Location) -> bool {
	let mut player_used_fastest_transport_method = true;
	match used_timetable_card {
		TimetableCard::LowSpeed => {
			if !current_location.get_high_speed_connections().is_empty() && (player.timetable_cards.contains(&TimetableCard::HighSpeed) || player.timetable_cards.contains(&TimetableCard::Joker)) {
				player_used_fastest_transport_method = false;
			}
			if !current_location.get_plane_connections().is_empty() && (player.timetable_cards.contains(&TimetableCard::Plane) || player.timetable_cards.contains(&TimetableCard::Joker)) {
				player_used_fastest_transport_method = false;
			}
		},
		TimetableCard::HighSpeed => {
			if !current_location.get_plane_connections().is_empty() && (player.timetable_cards.contains(&TimetableCard::Plane) || player.timetable_cards.contains(&TimetableCard::Joker)) {
				player_used_fastest_transport_method = false;
			}
		},
		_ => (),
	};
	return player_used_fastest_transport_method;
}

fn player_can_leave_country(player: &Player) -> bool {
	let mut can_leave_country = false;
	for timetable_card in &player.timetable_cards {
		match timetable_card {
			TimetableCard::LowSpeed => {
				for connection in player.current_location.get_low_speed_connections() {
					if Country::from(connection) != Country::from(player.current_location) {
						can_leave_country = true;
					}
				}
			},
			TimetableCard::HighSpeed => {
				for connection in player.current_location.get_high_speed_connections() {
					if Country::from(connection) != Country::from(player.current_location) {
						can_leave_country = true;
					}
				}
			},
			TimetableCard::Plane => {
				for connection in player.current_location.get_plane_connections() {
					if Country::from(connection) != Country::from(player.current_location) {
						can_leave_country = true;
					}
				}
			},
			TimetableCard::Joker => {
				for connection in player.current_location.get_joker_connections() {
					if Country::from(connection) != Country::from(player.current_location) {
						can_leave_country = true;
					}
				}
			},
		}
	}
	return can_leave_country;
}

fn is_move_possible(player: &Player) -> bool {
	let mut move_possible = false;

	for timetable_card in &player.timetable_cards {
		match timetable_card {
			TimetableCard::LowSpeed => move_possible = !player.current_location.get_low_speed_connections().is_empty(),
			TimetableCard::HighSpeed => move_possible = !player.current_location.get_high_speed_connections().is_empty(),
			TimetableCard::Plane => move_possible = !player.current_location.get_plane_connections().is_empty(),
			TimetableCard::Joker => move_possible = !player.current_location.get_joker_connections().is_empty(),
		}

		if move_possible {
			break;
		}
	}
	return move_possible;
}

fn player_wants_to_move_space_occupied_by_chaser(players: &Vec<Player>, runner: PlayerId, next_location: Location) -> bool {
	return players.iter()
		.filter(|x| x.id != runner)
		.filter(|x| x.current_location == next_location)
		.count() > 0;
}

fn remove_used_timetable_card_from_player(mut player: Player, timetable_card_used: &TimetableCard) -> Player {
	let mut already_removed = false;
	player.timetable_cards.retain(|x| if x != timetable_card_used || already_removed {
		true
	} else {
		already_removed = true;
		false
	});
	return player;
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
	pub new_location_already_sent: bool,
	pub use_timetable_card_already_sent: bool,
	pub event_card_bought: bool,
	pub stealth_mode_enabled: bool,
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