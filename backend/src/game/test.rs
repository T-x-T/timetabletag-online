use super::*;

mod create {
	use super::*;

	#[test]
	fn properly_adds_first_player() {
		let display_name = "test".to_string();
		let res = Game::create(display_name.clone());

		assert_eq!(res.players.clone().first().unwrap().display_name, display_name);
		assert_eq!(res.players.clone().first().unwrap().id, res.host);
	}

	#[test]
	fn destination_random_enough() {
		let n = 2_500; //passed 10k test runs with 2500, lower might be flakey

		let mut output: BTreeMap<String, usize> = BTreeMap::new();
		for _ in 0..n {
			let destination = Game::create(String::new()).destination;
			output.entry(destination).and_modify(|x| *x += 1).or_insert(1);
		}

		assert_eq!(output.len(), 5);
		assert!(output.iter().map(|x| *x.1).min().unwrap() > n / 6);
		assert!(output.iter().map(|x| *x.1).max().unwrap() < n / 4);
	}
}

mod join {
	use super::*;

	#[test]
	fn join_adds_2nd_player() {
		let mut game = Game::create("test_1".to_string());

		let player_id = game.join("test_2".to_string());

		assert_eq!(game.players.iter().nth(1).unwrap().display_name, "test_2".to_string());
		assert_eq!(game.players.iter().nth(1).unwrap().id, player_id.unwrap());
	}

	#[test]
	fn join_adds_3rd_player() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());

		let player_id = game.join("test_3".to_string());

		assert_eq!(game.players.iter().nth(2).unwrap().display_name, "test_3".to_string());
		assert_eq!(game.players.iter().nth(2).unwrap().id, player_id.unwrap());
	}

	#[test]
	fn join_adds_4th_player() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());

		let player_id = game.join("test_4".to_string());

		assert_eq!(game.players.iter().nth(3).unwrap().display_name, "test_4".to_string());
		assert_eq!(game.players.iter().nth(3).unwrap().id, player_id.unwrap());
	}

	#[test]
	fn join_doesnt_add_5th_player() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.join("test_4".to_string());

		let player_id = game.join("test_5".to_string());

		assert!(player_id.is_err());
		matches!(player_id.err().unwrap().downcast_ref::<crate::CustomError>(), Some(&crate::CustomError::LobbyFull));
	}
}

mod start {
	use super::*;

	#[test]
	fn start_with_3_players() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());

		let res = game.start(game.host);

		assert!(res.is_ok());
	}

	#[test]
	fn start_with_4_players() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.join("test_4".to_string());

		let res = game.start(game.host);

		assert!(res.is_ok());
	}

	#[test]
	fn start_with_2_players_fails() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());

		let res = game.start(game.host);

		assert!(res.is_err());
		matches!(res.err().unwrap().downcast_ref::<crate::CustomError>(), Some(&crate::CustomError::LobbyNotFullEnough));
	}

	#[test]
	fn start_already_started_game_fails() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.start(game.host);

		let res = game.start(game.host);

		assert!(res.is_err());
		matches!(res.err().unwrap().downcast_ref::<crate::CustomError>(), Some(&crate::CustomError::InvalidGameState));
	}

	#[test]
	fn other_player_cant_start() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let player = game.join("test_3".to_string());

		let res = game.start(player.unwrap());

		assert!(res.is_err());
		matches!(res.err().unwrap().downcast_ref::<crate::CustomError>(), Some(&crate::CustomError::ActionNotAllowed));
	}

	#[test]
	fn set_state_to_in_progress() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.start(game.host);
		
		assert_eq!(game.state, GameState::InProgress);
	}

	#[test]
	fn set_runner_randomly() {
		let n = 2_500; //passed 10k test runs with 2500, lower might be flakey

		let mut output: BTreeMap<String, usize> = BTreeMap::new();
		for _ in 0..n {
			let mut game = Game::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let _ = game.start(game.host);

			output.entry(game.runner.unwrap().display_name).and_modify(|x| *x += 1).or_insert(1);
		}

		assert_eq!(output.len(), 3);
		assert!(output.iter().map(|x| *x.1).min().unwrap() > n / 4);
		assert!(output.iter().map(|x| *x.1).max().unwrap() < n / 2);
	}

	#[test]
	fn runner_gets_first_turn() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.start(game.host);
		
		assert_eq!(game.runner.unwrap(), game.current_turn.unwrap());
	}

	#[test]
	fn each_player_gets_5_cards() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.start(game.host);
		
		game.timetable_cards.into_iter().for_each(|x| {
			assert_eq!(x.1.len(), 5);
		});
	}

	#[test]
	fn timetable_card_stack_gets_filled() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.start(game.host);

		//85 because 100 - 3 players * 5 cards = 85
		assert_eq!(game.timetable_card_stack.len(), 85);
	}

	#[test]
	fn event_card_stack_gets_filled() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.start(game.host);

		assert_eq!(game.event_card_stack.len(), 20);
	}
}

mod make_move {
	use super::*;

	#[test]
	fn returns_error_when_game_hasnt_started() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());

		let move_made = Move {
			player_id: game.host,
			..Default::default()
		};

		let res = game.make_move(move_made);

		assert!(res.is_err());
		matches!(res.err().unwrap().downcast_ref::<crate::CustomError>(), Some(&crate::CustomError::NotYourTurn));
	}

	#[test]
	fn sets_in_progress_move_when_its_none() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.start(game.host);

		game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });

		let move_made = Move {
			player_id: game.host,
			..Default::default()
		};

		let _ = game.make_move(move_made);
		assert!(game.in_progress_move.is_some());
	}

	mod turn_logic {
		use super::*;

		#[test]
		fn returns_error_when_wrong_player_makes_turn() {
			let mut game = Game::create("test_1".to_string());
			let other_player = game.join("test_2".to_string()).unwrap();
			let _ = game.join("test_3".to_string());
			let _ = game.start(game.host);

			println!("{game:?}");

			let move_made = Move {
				player_id: if game.current_turn.as_ref().unwrap().id == game.host {other_player} else {game.host},
				..Default::default()
			};

			let res = game.make_move(move_made);

			assert!(res.is_err());
			matches!(res.err().unwrap().downcast_ref::<crate::CustomError>(), Some(&crate::CustomError::NotYourTurn));
		}

		#[test]
		fn current_turn_gets_set_to_next_player_1() {
			let mut game = Game::create("test_1".to_string());
			let player2 = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let _ = game.start(game.host);
	
			game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
	
			let _ = game.make_move(Move { player_id: game.host, finish_move: true, ..Default::default()});
	
			assert_eq!(game.current_turn.unwrap().id, player2.unwrap());
		}
	
		#[test]
		fn current_turn_gets_set_to_next_player_2() {
			let mut game = Game::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let _ = game.start(game.host);
	
			game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
	
			let _ = game.make_move(Move { player_id: game.host, finish_move: true, ..Default::default()});
			let _ = game.make_move(Move { player_id: player2, finish_move: true, ..Default::default()});
	
			assert_eq!(game.current_turn.unwrap().id, player3);
		}
	
		#[test]
		fn current_turn_gets_set_to_next_player_3() {
			let mut game = Game::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let _ = game.start(game.host);
	
			game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
	
			let _ = game.make_move(Move { player_id: game.host, finish_move: true, ..Default::default()});
			let _ = game.make_move(Move { player_id: player2, finish_move: true, ..Default::default()});
			let _ = game.make_move(Move { player_id: player3, finish_move: true, ..Default::default()});
	
			assert_eq!(game.current_turn.unwrap().id, game.host);
		}
	}

	mod next_location_handling {
		use super::*;

		#[test]
		fn next_location_gets_parsed_when_set() {
			let mut game = Game::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let _ = game.start(game.host);
	
			game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
			game.runner = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });

			let move_made = Move {
				player_id: game.host,
				next_location: Some("paris".to_string()),
				..Default::default()
			};
	
			let _ = game.make_move(move_made);

			assert_eq!(game.in_progress_move.unwrap().move_data.next_location_parsed.unwrap(), Location::Paris);
		}

		#[test]
		fn use_timetable_card_gets_parsed_when_set() {
			let mut game = Game::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let _ = game.start(game.host);
	
			game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
			game.runner = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("joker".to_string()),
				..Default::default()
			};
	
			let _ = game.make_move(move_made);

			assert_eq!(game.in_progress_move.unwrap().move_data.use_timetable_card_parsed.unwrap(), TimetableCard::Joker);
		}

		#[test]
		fn new_location_already_sent_gets_set_in_in_progress_move() {
			let mut game = Game::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let _ = game.start(game.host);
	
			game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
			game.runner = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });

			let move_made = Move {
				player_id: game.host,
				next_location: Some("paris".to_string()),
				..Default::default()
			};
	
			let _ = game.make_move(move_made);

			assert!(game.in_progress_move.unwrap().new_location_already_sent);
		}

		#[test]
		fn use_timetable_card_already_sent_gets_set_in_in_progress_move() {
			let mut game = Game::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let _ = game.start(game.host);
	
			game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
			game.runner = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("joker".to_string()),
				..Default::default()
			};
	
			let _ = game.make_move(move_made);

			assert!(game.in_progress_move.unwrap().use_timetable_card_already_sent);
		}
	}

	#[test]
	fn return_error_when_player_doesnt_have_right_timetable_card() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.start(game.host);

		game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
		game.runner = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
		game.timetable_cards.entry(game.host).and_modify(|x| *x = vec![TimetableCard::LowSpeed; 5]);

		let move_made = Move {
			player_id: game.host,
			use_timetable_card: Some("joker".to_string()),
			next_location: Some("paris".to_string()),
			..Default::default()
		};

		let res = game.make_move(move_made);
		assert!(res.is_err());
		matches!(res.err().unwrap().downcast_ref::<crate::CustomError>(), Some(&crate::CustomError::MissingCard));
	}

	#[test]
	fn returns_error_when_player_cant_get_to_next_location() {
		let mut game = Game::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.start(game.host);

		game.current_turn = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
		game.runner = Some(Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy });
		game.timetable_cards.entry(game.host).and_modify(|x| *x = vec![TimetableCard::LowSpeed; 5]);

		let move_made = Move {
			player_id: game.host,
			use_timetable_card: Some("low_speed".to_string()),
			next_location: Some("berlin".to_string()),
			..Default::default()
		};

		let res = game.make_move(move_made);
		assert!(res.is_err());
		matches!(res.err().unwrap().downcast_ref::<crate::CustomError>(), Some(&crate::CustomError::InvalidNextLocation));
	}
}