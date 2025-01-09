use super::*;

#[derive(Debug, Clone)]
pub struct Lobby {
	pub id: GameId,
	pub invite_code: String,
	pub host: PlayerId,
	pub players: Vec<Player>,
}

impl Lobby {
	pub fn create(display_name: String) -> Self {
		let mut rng = thread_rng();
		let invite_code_part1 = rng.gen_range(0..=999);
		let invite_code_part2 = rng.gen_range(0..=999);

		let player_id = PlayerId::new_v4();
		let player = Player {
			id: player_id.clone(),
			display_name,
			current_location: Location::Nancy,
			timetable_cards: Vec::new(),
		};

		return Self {
			id: GameId::new_v4(),
			invite_code: format!("{invite_code_part1:0>3}-{invite_code_part2:0>3}"), //TODO: collision possible
			host: player_id,
			players: vec![player],
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
			timetable_cards: Vec::new(),
		};

		self.players.push(player);
		
		return Ok(id);
	}

	pub fn start(&mut self, player_id: PlayerId) -> Result<InProgressGame, Box<dyn Error>> {
		if player_id != self.host {
			return Err(Box::new(crate::CustomError::ActionNotAllowed));
		}
		
		if self.players.len() <= 2 {
			return Err(Box::new(crate::CustomError::LobbyNotFullEnough));
		}
		
		let mut rng = thread_rng();
		let rand_player_id = rng.gen_range(0..=self.players.len() - 1);
		let rand_destination_index = rng.gen_range(0..=4);

		let runner = self.players.iter().nth(rand_player_id).unwrap().clone().id;

		let mut game = InProgressGame {
			id: self.id,
			host: self.host,
			runner: runner,
			players: self.players.clone(),
			destination: ["dublin", "copenhagen", "vienna", "rome", "madrid"].into_iter().nth(rand_destination_index).unwrap().into(),
			current_turn: runner,
			coins_runner: 0,
			coins_chasers: 0,
			last_used_timetable_card: None,
			dice_result: None,
			event_card_bought: false,
			runner_path: vec![],
			in_progress_move: None,
			timetable_card_stack: generate_timetable_card_stack(),
			event_card_stack: generate_event_card_stack(),
		};		

		game.players = self.players.clone().into_iter().map(|mut x| {
			x.timetable_cards = vec![game.timetable_card_stack.pop().unwrap(), game.timetable_card_stack.pop().unwrap(), game.timetable_card_stack.pop().unwrap(), game.timetable_card_stack.pop().unwrap(), game.timetable_card_stack.pop().unwrap()];
			return x;
		}).collect();

		return Ok(game);
	}
}
