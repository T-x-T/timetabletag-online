use super::*;
use super::lobby::Lobby;
use super::in_progress_game::Move;

mod create {
	use super::*;

	#[test]
	fn properly_adds_first_player() {
		let display_name = "test".to_string();
		let res = Lobby::create(display_name.clone());

		assert_eq!(res.players.clone().first().unwrap().display_name, display_name);
		assert_eq!(res.players.clone().first().unwrap().id, res.host);
	}
}

mod join {
	use super::*;

	#[test]
	fn join_adds_2nd_player() {
		let mut game = Lobby::create("test_1".to_string());

		let player_id = game.join("test_2".to_string());

		assert_eq!(game.players.iter().nth(1).unwrap().display_name, "test_2".to_string());
		assert_eq!(game.players.iter().nth(1).unwrap().id, player_id.unwrap());
	}

	#[test]
	fn join_adds_3rd_player() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());

		let player_id = game.join("test_3".to_string());

		assert_eq!(game.players.iter().nth(2).unwrap().display_name, "test_3".to_string());
		assert_eq!(game.players.iter().nth(2).unwrap().id, player_id.unwrap());
	}

	#[test]
	fn join_adds_4th_player() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());

		let player_id = game.join("test_4".to_string());

		assert_eq!(game.players.iter().nth(3).unwrap().display_name, "test_4".to_string());
		assert_eq!(game.players.iter().nth(3).unwrap().id, player_id.unwrap());
	}

	#[test]
	fn join_doesnt_add_5th_player() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.join("test_4".to_string());

		let res = game.join("test_5".to_string());

		assert!(res.is_err());
		assert_eq!(res.err().unwrap().to_string(), crate::CustomError::LobbyFull.to_string());
	}
}

mod start {
	use super::*;

	#[test]
	fn start_with_3_players() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());

		let res = game.start(game.host);

		assert!(res.is_ok());
	}

	#[test]
	fn start_with_4_players() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let _ = game.join("test_4".to_string());

		let res = game.start(game.host);

		assert!(res.is_ok());
	}

	#[test]
	fn start_with_2_players_fails() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());

		let res = game.start(game.host);

		assert!(res.is_err());
		assert_eq!(res.err().unwrap().to_string(), crate::CustomError::LobbyNotFullEnough.to_string());
	}

	#[test]
	fn other_player_cant_start() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let player = game.join("test_3".to_string());

		let res = game.start(player.unwrap());

		assert!(res.is_err());
		assert_eq!(res.err().unwrap().to_string(), crate::CustomError::ActionNotAllowed.to_string());
	}

	#[test]
	fn set_runner_randomly() {
		let n = 2_500; //passed 10k test runs with 2500, lower might be flakey

		let mut output: BTreeMap<String, usize> = BTreeMap::new();
		for _ in 0..n {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let game = game.start(game.host).unwrap();

			output.entry(game.players.iter().filter(|x| x.id == game.runner).next().unwrap().display_name.clone()).and_modify(|x| *x += 1).or_insert(1);
		}

		assert_eq!(output.len(), 3);
		assert!(output.iter().map(|x| *x.1).min().unwrap() > n / 4);
		assert!(output.iter().map(|x| *x.1).max().unwrap() < n / 2);
	}

	#[test]
	fn runner_gets_first_turn() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let game = game.start(game.host).unwrap();
		
		assert_eq!(game.runner, game.current_turn);
	}

	#[test]
	fn each_player_gets_5_cards() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let game = game.start(game.host).unwrap();
		
		game.players.into_iter().for_each(|x| assert_eq!(x.timetable_cards.len(), 5));
	}

	#[test]
	fn timetable_card_stack_gets_filled() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let game = game.start(game.host).unwrap();

		//85 because 100 - 3 players * 5 cards = 85
		assert_eq!(game.timetable_card_stack.len(), 85);
	}

	#[test]
	fn event_card_stack_gets_filled() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let game = game.start(game.host).unwrap();

		assert_eq!(game.event_card_stack.len(), 20);
	}

	#[test]
	fn destination_random_enough() {
		let n = 2_500; //passed 10k test runs with 2500, lower might be flakey

		let mut output: BTreeMap<Location, usize> = BTreeMap::new();
		for _ in 0..n {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let game = game.start(game.host).unwrap();

			output.entry(game.destination).and_modify(|x| *x += 1).or_insert(1);
		}

		assert_eq!(output.len(), 5);
		assert!(output.iter().map(|x| *x.1).min().unwrap() > n / 6);
		assert!(output.iter().map(|x| *x.1).max().unwrap() < n / 4);
	}
}

mod make_move {
	use super::*;

	#[test]
	fn sets_in_progress_move_when_its_none() {
		let mut game = Lobby::create("test_1".to_string());
		let _ = game.join("test_2".to_string());
		let _ = game.join("test_3".to_string());
		let mut game = game.start(game.host).unwrap();

		game.current_turn = game.host;

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
			let mut game = Lobby::create("test_1".to_string());
			let other_player = game.join("test_2".to_string()).unwrap();
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			println!("{game:?}");

			let move_made = Move {
				player_id: if game.current_turn == game.host {other_player} else {game.host},
				..Default::default()
			};

			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::NotYourTurn.to_string());
		}

		#[test]
		fn current_turn_gets_set_to_next_player_1() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();
	
			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			game.current_turn = game.host;
			game.get_another_turn = false;

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				finish_move: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_ok());
			assert_eq!(game.current_turn, player2);
		}
	
		#[test]
		fn current_turn_gets_set_to_next_player_2() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();
	
			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			game.current_turn = game.host;
			game.get_another_turn = false;

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				finish_move: true,
				..Default::default()
			};
			let _ = game.make_move(move_made);

			let move_made = Move {
				player_id: player2,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("dijon".to_string()),
				finish_move: true,
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert_eq!(game.current_turn, player3);
		}
	
		#[test]
		fn current_turn_gets_set_to_next_player_3() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();
	
			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			game.current_turn = game.host;
			game.get_another_turn = false;

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				finish_move: true,
				..Default::default()
			};
			let _ = game.make_move(move_made);

			let move_made = Move {
				player_id: player2,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("dijon".to_string()),
				finish_move: true,
				..Default::default()
			};
			let _ = game.make_move(move_made);
			
			let move_made = Move {
				player_id: player3,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("luxembourg".to_string()),
				finish_move: true,
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert_eq!(game.current_turn, game.host);
		}

		#[test]
		fn runner_gets_two_turns_at_start() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let _ = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();
			game.timetable_card_stack = vec![TimetableCard::LowSpeed; 10];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				finish_move: true,
				..Default::default()
			};
			let res = game.make_move(move_made);
			assert!(res.is_ok());

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("le_havre".to_string()),
				finish_move: true,
				..Default::default()
			};
			let res = game.make_move(move_made);
			assert!(res.is_ok());

			assert_eq!(game.current_turn, player2);
		}

		#[test]
		fn returns_error_when_finishing_move_without_moving() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string()).unwrap();
			let _ = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				finish_move: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::ActionNotAllowed.to_string());
		}
	}

	mod next_location_handling {
		use super::*;

		#[test]
		fn next_location_gets_parsed_when_set() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();
	
			game.current_turn = game.host;
			game.runner = game.host;

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
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();
	
			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("joker".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
	
			let _ = game.make_move(move_made);

			assert_eq!(game.in_progress_move.unwrap().move_data.use_timetable_card_parsed.unwrap(), TimetableCard::Joker);
		}

		#[test]
		fn new_location_already_sent_gets_set_in_in_progress_move() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();
	
			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				next_location: Some("paris".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				..Default::default()
			};
	
			let _ = game.make_move(move_made);

			assert!(game.in_progress_move.unwrap().new_location_already_sent);
		}

		#[test]
		fn use_timetable_card_already_sent_gets_set_in_in_progress_move() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();
	
			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
	
			let _ = game.make_move(move_made);

			assert!(game.in_progress_move.unwrap().use_timetable_card_already_sent);
		}

		#[test]
		fn return_error_when_player_doesnt_have_right_timetable_card() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("joker".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};

			let res = game.make_move(move_made);
			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::MissingTimetableCard.to_string());
		}

		#[test]
		fn returns_error_when_player_cant_get_to_next_location() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("berlin".to_string()),
				..Default::default()
			};

			let res = game.make_move(move_made);
			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::InvalidNextLocation.to_string());
		}

		#[test]
		fn returns_error_when_player_already_moved_this_round() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_ok());
			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("calais".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);
			
			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::AlreadyMoved.to_string());
		}

		#[test]
		fn player_location_get_updated() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);
			
			assert_eq!(game.players.first().unwrap().current_location, Location::Paris);
		}

		#[test]
		fn runner_path_gets_updated_for_runner() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert_eq!(game.runner_path.first().unwrap(), &Location::Paris);
		}

		#[test]
		fn runner_path_doesnt_get_updated_for_chaser() {
			let mut game = Lobby::create("test_1".to_string());
			let player1 = game.join("test_2".to_string()).unwrap();
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player1;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: player1,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert!(game.runner_path.is_empty());
		}

		#[test]
		fn last_used_timetable_card_gets_updated() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert_eq!(game.last_used_timetable_card.unwrap(), TimetableCard::LowSpeed);
		}

		#[test]
		fn player_gets_new_card() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();
			game.timetable_card_stack = vec![TimetableCard::Joker];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert_eq!(game.players.iter().find(|x| x.id == game.runner).unwrap().timetable_cards.len(), 5);
			assert_eq!(game.players.iter().find(|x| x.id == game.runner).unwrap().timetable_cards.iter().filter(|x| **x == TimetableCard::Joker).count(), 1);
		}

		#[test]
		fn chaser_cant_move_to_location_of_other_chaser() {
			let mut game = Lobby::create("test_1".to_string());
			let chaser1 = game.join("test_2".to_string()).unwrap();
			let chaser2 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker; 10];

			game.players = vec![
				Player { id: game.runner.clone(), display_name: "test_1".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: chaser1, display_name: "test_2".to_string(), current_location: Location::Paris, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: chaser2, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			game.current_turn = chaser2;
			let move_made = Move {
				player_id: chaser2,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::InvalidNextLocation.to_string());
		}

		#[test]
		fn timetable_cards_received_gets_returned() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string());
			let _ = game.join("test_3".to_string());
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();
			game.timetable_card_stack = vec![TimetableCard::Joker];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made).unwrap();

			assert_eq!(res.timetable_cards_received, vec![TimetableCard::Joker]);
		}
	}

	mod coin_fields {
		use super::*;

		#[test]
		fn runner_coins_get_updated_when_landing_on_coin_field() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Paris, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("le_havre".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert!(game.coins_runner > 0);
		}

		#[test]
		fn runner_coins_stay_zero_when_landing_on_non_coin_field() {
			let mut game = Lobby::create("test_1".to_string());
			let _ = game.join("test_2".to_string()).unwrap();
			let _ = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();
			game.timetable_card_stack = vec![TimetableCard::Joker];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert_eq!(game.coins_runner, 0);
		}

		#[test]
		fn chaser_coins_get_updated_when_landing_on_coin_field() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Paris, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: player2,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("le_havre".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert!(game.coins_chasers > 0);
		}


		#[test]
		fn chaser_coins_stay_zero_when_landing_on_non_coin_field() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let _ = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.players = game.players.clone().into_iter().map(|mut x| {
				x.timetable_cards = vec![TimetableCard::LowSpeed; 5];
				return x;
			}).collect();
			game.timetable_card_stack = vec![TimetableCard::Joker];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("paris".to_string()),
				..Default::default()
			};
			let _ = game.make_move(move_made);

			assert_eq!(game.coins_chasers, 0);
		}

		#[test]
		fn coin_randomness_is_random_enough() {
			let mut output: BTreeMap<i32, i32> = BTreeMap::new();
			let n = 3_000;

			for _ in 0..=n {
				let mut game = Lobby::create("test_1".to_string());
				let player2 = game.join("test_2".to_string()).unwrap();
				let player3 = game.join("test_3".to_string()).unwrap();
				let mut game = game.start(game.host).unwrap();
	
				game.current_turn = game.host;
				game.runner = game.host;
				game.timetable_card_stack = vec![TimetableCard::Joker];
	
				game.players = vec![
					Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Paris, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
					Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
					Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				];
	
				let move_made = Move {
					player_id: game.host,
					use_timetable_card: Some("low_speed".to_string()),
					next_location: Some("le_havre".to_string()),
					..Default::default()
				};
				let _ = game.make_move(move_made);

				output.entry(game.coins_runner as i32).and_modify(|x| *x += 1).or_insert(1);
			}

			assert_eq!(output.len(), 6);
			assert!(*output.get(&1).unwrap() > n / 7 && *output.get(&1).unwrap() < n / 5);
			assert!(*output.get(&2).unwrap() > n / 7 && *output.get(&2).unwrap() < n / 5);
			assert!(*output.get(&3).unwrap() > n / 7 && *output.get(&3).unwrap() < n / 5);
			assert!(*output.get(&4).unwrap() > n / 7 && *output.get(&4).unwrap() < n / 5);
			assert!(*output.get(&5).unwrap() > n / 7 && *output.get(&5).unwrap() < n / 5);
			assert!(*output.get(&6).unwrap() > n / 7 && *output.get(&6).unwrap() < n / 5);
		}


		#[test]
		fn number_of_received_coins_returned() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Paris, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("le_havre".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made).unwrap();

			assert!(res.coins_received.unwrap() > 0);
			assert_eq!(res.coins_received.unwrap(), game.coins_runner);
		}
	}

	mod winning {
		use super::*;

		#[test]
		fn runner_doesnt_win_when_getting_to_dest_without_10_coins() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker; 5];
			game.destination = Location::Madrid;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Zaragoza, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("madrid".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_ok());
			assert!(res.unwrap().finished_game.is_none());
		}

		#[test]
		fn runner_wins_when_getting_to_dest_with_10_coins() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker; 5];
			game.destination = Location::Madrid;
			game.coins_runner = 10;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Zaragoza, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.host,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("madrid".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_ok());
			assert!(res.as_ref().unwrap().finished_game.is_some());
			assert_eq!(res.as_ref().unwrap().clone().finished_game.unwrap().win_condition, WinCondition::GotToDestination);
			assert_eq!(res.unwrap().finished_game.unwrap().winning_team, Team::Runner);
		}

		#[test]
		fn chasers_win_when_catching_runner() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker; 5];
			game.destination = Location::Madrid;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Zaragoza, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Madrid, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: player2,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("zaragoza".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);
			assert!(res.is_ok());

			assert!(res.is_ok());
			assert!(res.as_ref().unwrap().finished_game.is_some());
			assert_eq!(res.as_ref().unwrap().clone().finished_game.unwrap().win_condition, WinCondition::RunnerCaught);
			assert_eq!(res.unwrap().finished_game.unwrap().winning_team, Team::Chaser);
		}

		#[test]
		fn catching_runner_returns_runner_caught_true() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker; 5];
			game.destination = Location::Madrid;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Zaragoza, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Madrid, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: player2,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("zaragoza".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);
			assert!(res.is_ok());

			assert!(res.unwrap().runner_caught);
		}

		#[test]
		fn chasers_win_when_card_stack_is_emptied() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.runner;
			game.timetable_card_stack = vec![];
			game.destination = Location::Madrid;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Zaragoza, timetable_cards: vec![TimetableCard::LowSpeed], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Madrid, timetable_cards: vec![TimetableCard::LowSpeed], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed], ..Default::default() },
			];

			let move_made = Move {
				player_id: player2,
				use_timetable_card: Some("low_speed".to_string()),
				next_location: Some("albacete".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_ok());
			assert!(res.as_ref().unwrap().finished_game.is_some());
			assert_eq!(res.as_ref().unwrap().clone().finished_game.unwrap().win_condition, WinCondition::TimetableCardsRanOut);
			assert_eq!(res.unwrap().finished_game.unwrap().winning_team, Team::Chaser);
		}
	}

	mod powerup {
		use super::*;

		#[test]
		fn buying_runner_country_works() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Padua, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				buy_powerup: Some("learn_runner_country".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made).unwrap();

			assert_eq!(res.power_up_status.runner_country.unwrap(), Country::Italy);
		}

		#[test]
		fn buying_runner_country_doesnt_work_when_too_few_coins() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 4;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Padua, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				buy_powerup: Some("learn_runner_country".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::NotEnoughCoins.to_string());
		}

		#[test]
		fn buying_runner_country_doesnt_work_when_too_few_coins_for_three_chasers() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let player4 = game.join("test_4".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 9;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Padua, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player4, display_name: "test_4".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				buy_powerup: Some("learn_runner_country".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::NotEnoughCoins.to_string());
		}

		#[test]
		fn buying_runner_country_doesnt_do_anything_when_runner_buys_it() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Padua, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				buy_powerup: Some("learn_runner_country".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made).unwrap();

			assert!(res.power_up_status.runner_country.is_none());
		}

		#[test]
		fn buying_runner_location_works() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 10;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Padua, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				buy_powerup: Some("learn_runner_location".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made).unwrap();

			assert_eq!(res.power_up_status.runner_location.unwrap(), Location::Padua);
		}

		#[test]
		fn buying_chaser_two_turns_works() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 100;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Padua, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				buy_powerup: Some("chaser_gets_two_turns".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made).unwrap();
			assert!(res.power_up_status.get_another_turn);
			assert!(game.get_another_turn);


			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("paris".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				finish_move: true,
				..Default::default()
			};
			let _ = game.make_move(move_made).unwrap();

			assert_eq!(game.current_turn, player2);

			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("le_havre".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				finish_move: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(!game.get_another_turn);
			assert!(res.is_ok());
			assert_eq!(game.current_turn, player3);
		}

		#[test]
		fn buying_runner_destination_works() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.destination = Location::Dublin;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 100;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Padua, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				buy_powerup: Some("learn_runner_destination".to_string()),
				..Default::default()
			};
			let res = game.make_move(move_made).unwrap();

			assert_eq!(res.power_up_status.runner_destination.unwrap(), Location::Dublin);
		}
	}

	mod buy_event_card {
		use super::*;

		#[test]
		fn works() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;
			game.coins_runner = 5;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Rennes, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("brest".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_ok());
			assert!(res.as_ref().unwrap().event_card_received.is_some());
			assert!(res.as_ref().unwrap().event_card_bought);
			assert!(res.as_ref().unwrap().event_card_received.is_some());
			assert_eq!(game.players.iter().find(|x| x.id == game.host).unwrap().event_cards.len(), 1);
			assert!(game.in_progress_move.unwrap().event_card_bought);
			assert_eq!(game.players.iter().find(|x| x.id == game.host).unwrap().event_cards.first().unwrap(), res.as_ref().unwrap().event_card_received.as_ref().unwrap());
		}

		#[test]
		fn doesnt_work_when_new_location_didnt_get_sent() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;
			game.coins_runner = 5;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Rennes, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::EventCardNoLocationSent.to_string());
		}

		#[test]
		fn doesnt_work_when_event_card_already_bought() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;
			game.coins_runner = 5;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Rennes, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("brest".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);
			assert!(res.is_ok());
			
			let move_made = Move {
				player_id: game.current_turn,
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::EventCardAlreadyBought.to_string());
		}

		#[test]
		fn doesnt_work_when_not_on_event_field() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;
			game.coins_runner = 5;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Rennes, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("nantes".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::NotAnEventField.to_string());
		}

		#[test]
		fn doesnt_work_if_runner_has_no_coins() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;
			game.coins_runner = 0;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Rennes, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("brest".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::NotEnoughCoins.to_string());
		}

		#[test]
		fn doesnt_work_if_chaser_has_no_coins() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = player2;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 0;
			game.coins_runner = 5;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Rennes, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("brest".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::NotEnoughCoins.to_string());
		}

		#[test]
		fn event_card_stack_gets_smaller_by_one() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;
			game.coins_runner = 5;

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Rennes, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("brest".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_ok());
			assert_eq!(game.event_card_stack.len(), 19);
		}

		#[test]
		fn returns_event_card_empty_error_when_its_empty() {
			let mut game = Lobby::create("test_1".to_string());
			let player2 = game.join("test_2".to_string()).unwrap();
			let player3 = game.join("test_3".to_string()).unwrap();
			let mut game = game.start(game.host).unwrap();

			game.current_turn = game.host;
			game.runner = game.host;
			game.timetable_card_stack = vec![TimetableCard::Joker];
			game.coins_chasers = 5;
			game.coins_runner = 5;
			game.event_card_stack = Vec::new();

			game.players = vec![
				Player { id: game.host, display_name: "test_1".to_string(), current_location: Location::Rennes, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player2, display_name: "test_2".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
				Player { id: player3, display_name: "test_3".to_string(), current_location: Location::Nancy, timetable_cards: vec![TimetableCard::LowSpeed; 5], ..Default::default() },
			];

			let move_made = Move {
				player_id: game.current_turn,
				next_location: Some("brest".to_string()),
				use_timetable_card: Some("low_speed".to_string()),
				buy_event_card: true,
				..Default::default()
			};
			let res = game.make_move(move_made);

			assert!(res.is_err());
			assert_eq!(res.err().unwrap().to_string(), crate::CustomError::EventCardStackEmpty.to_string());
		}
	}


}