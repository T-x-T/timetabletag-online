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
}