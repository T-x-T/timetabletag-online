use uuid::Uuid;
use std::collections::BTreeMap;
use rand::prelude::*;

type GameId = Uuid;
type PlayerId = Uuid;

#[derive(Debug, Clone)]
pub struct Game {
	id: GameId,
	host: PlayerId,
	state: GameState,
	runner: Option<Player>,
	players: Vec<Player>,
	destination: String,
	current_turn: Option<Player>,
	coins_runner: usize,
	coins_chasers: usize,
	timetable_cards: BTreeMap<PlayerId, Vec<String>>,
	last_used_timetable_card: Option<String>,
	dice_result: Option<usize>,
	event_card_bought: bool,
	winning_team: Option<String>,
	win_condition: Option<String>,
	runner_path: Vec<String>,
}

impl Game {
	pub fn create(display_name: String) -> Self {
		let mut rng = thread_rng();
		let rand_destination_index = rng.gen_range(1..=5);

		let player_id = PlayerId::new_v4();
		let player = Player {
			id: player_id.clone(),
			display_name,
		};

		return Self {
			id: GameId::new_v4(),
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
		}
	}

	pub fn join(mut self, display_name: String) -> PlayerId {
		let id = PlayerId::new_v4();
		let player = Player {
			id: id.clone(),
			display_name,
		};

		self.players.push(player);
		
		return id;
	}

	pub fn make_move(mut self, move_made: Move) -> MoveResult {

		return MoveResult::default();
	}
}

#[derive(Debug, Clone, Copy)]
enum GameState {
	Lobby,
	InProgress,
	Finished,
}

#[derive(Debug, Clone)]
struct Player {
	id: Uuid,
	display_name: String,
}

#[derive(Debug, Clone)]
pub struct Move {
	player_id: PlayerId,
	next_location: Option<String>,
	use_card: Option<String>,
	buy_event_card: bool,
	use_event_card: Option<String>,
	buy_powerup: Option<String>,
	use_powerup: Option<String>,
	throw_timetable_cards_away: Vec<String>,
	finish_move: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MoveResult {
	coins_received: Option<usize>,
	event_card_received: Option<String>,
	event_card_bought: bool,
	runner_caught: bool,
	timetable_cards_received: Vec<String>,
}