#[cfg(test)]
mod test;

pub mod rest_api;

use uuid::Uuid;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Display;
use rand::prelude::*;

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
	last_used_timetable_card: Option<String>,
	dice_result: Option<u8>,
	event_card_bought: bool,
	winning_team: Option<String>,
	win_condition: Option<String>,
	runner_path: Vec<String>,
	in_progress_move: Option<Move>,
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

	pub fn make_move(&mut self, move_made: Move) -> Result<MoveResult, Box<dyn Error>> {
		if !self.current_turn.as_ref().is_some_and(|x| x.id == move_made.player_id) {
			return Err(Box::new(crate::CustomError::NotYourTurn));
		}
		
		if self.in_progress_move.is_none() {
			self.in_progress_move = Some(move_made);	
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
}

#[derive(Debug, Clone, serde::Deserialize)]
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

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct MoveResult {
	coins_received: Option<usize>,
	event_card_received: Option<String>,
	event_card_bought: bool,
	runner_caught: bool,
	timetable_cards_received: Vec<String>,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimetableCard {
	LowSpeed,
	HighSpeed,
	Plane,
	Joker,
}

impl Display for TimetableCard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TimetableCard::LowSpeed => write!(f, "low_speed"),
			TimetableCard::HighSpeed => write!(f, "high_speed"),
			TimetableCard::Plane => write!(f, "plane"),
			TimetableCard::Joker => write!(f, "joker"),
		}
	}
}

impl std::convert::From<String> for TimetableCard {
	fn from(value: String) -> Self {
		match value.as_str() {
			"low_speed" => TimetableCard::LowSpeed,
			"high_speed" => TimetableCard::HighSpeed,
			"plane" => TimetableCard::Plane,
			"joker" => TimetableCard::Joker,
			_ => panic!("{value} not a valid TimetableCard ID"),
		}
	}
}

// There are the following number of cards in the real game:
// low_speed:  50 = 50%
// high_speed: 30 = 30%
// plane:      16 = 16%
// joker:       4 =  4%
// total:     100 =100%
fn generate_timetable_card_stack() -> Vec<TimetableCard> {
	let mut rng = thread_rng();
	let mut output: Vec<TimetableCard> = Vec::new();
	
	for _ in 0..50 {
		output.push(TimetableCard::LowSpeed);
	}
	for _ in 0..30 {
		output.push(TimetableCard::HighSpeed);
	}
	for _ in 0..16 {
		output.push(TimetableCard::Plane);
	}
	for _ in 0..4 {
		output.push(TimetableCard::Joker);
	}

	output.shuffle(&mut rng);

	return output;
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventCard {
	GiveMeYourCards,
	HuntedByMenForSport,
	LuxembourgIsGermanyFrance,
	LetsGoToTheBeach,
	ImagineTrains,
	ConsiderVelocity,
	ItsPopsicle,
	HydrateOrDiedrate,
	StealthOutfit,
	CardinalDirectionsAndVibes,
	Pizzazz,
	RatMode,
	BingBong,
	LeaveCountryImmediately,
	ZugFaelltAus,
	SnackZone,
	ItsAllInTheTrees,
	BonjourToEveryone,
	NoTalk,
	SloveniaAsATreat,
}

impl Display for EventCard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EventCard::GiveMeYourCards => write!(f, "give_me_your_cards"),
			EventCard::HuntedByMenForSport => write!(f, "hunted_by_men_for_sport"),
			EventCard::LuxembourgIsGermanyFrance => write!(f, "luxembourg_is_germany_france"),
			EventCard::LetsGoToTheBeach => write!(f, "lets_go_to_the_beach"),
			EventCard::ImagineTrains => write!(f, "imagine_if_trains"),
			EventCard::ConsiderVelocity => write!(f, "consider_velocity"),
			EventCard::ItsPopsicle => write!(f, "its_popsicle"),
			EventCard::HydrateOrDiedrate => write!(f, "hydrate_or_diedrate"),
			EventCard::StealthOutfit => write!(f, "stealth_outfit"),
			EventCard::CardinalDirectionsAndVibes => write!(f, "cardinal_directions_and_vibes"),
			EventCard::Pizzazz => write!(f, "pizzazz"),
			EventCard::RatMode => write!(f, "rat_mode"),
			EventCard::BingBong => write!(f, "bing_bong"),
			EventCard::LeaveCountryImmediately => write!(f, "leave_country_immediately"),
			EventCard::ZugFaelltAus => write!(f, "zug_faellt_aus"),
			EventCard::SnackZone => write!(f, "snack_zone"),
			EventCard::ItsAllInTheTrees => write!(f, "its_all_in_the_trees"),
			EventCard::BonjourToEveryone => write!(f, "bonjour_to_everyone"),
			EventCard::NoTalk => write!(f, "no_talk"),
			EventCard::SloveniaAsATreat => write!(f, "slovenia_as_a_treat"),
		}
	}
}

impl std::convert::From<String> for EventCard {
	fn from(value: String) -> Self {
		match value.as_str() {
			"give_me_your_cards" => EventCard::GiveMeYourCards,
			"hunted_by_men_for_sport" => EventCard::HuntedByMenForSport,
			"luxembourg_is_germany_france" => EventCard::LuxembourgIsGermanyFrance,
			"lets_go_to_the_beach" => EventCard::LetsGoToTheBeach,
			"imagine_if_trains" => EventCard::ImagineTrains,
			"consider_velocity" => EventCard::ConsiderVelocity,
			"its_popsicle" => EventCard::ItsPopsicle,
			"hydrate_or_diedrate" => EventCard::HydrateOrDiedrate,
			"stealth_outfit" => EventCard::StealthOutfit,
			"cardinal_directions_and_vibes" => EventCard::CardinalDirectionsAndVibes,
			"pizzazz" => EventCard::Pizzazz,
			"rat_mode" => EventCard::RatMode,
			"bing_bong" => EventCard::BingBong,
			"leave_country_immediately" => EventCard::LeaveCountryImmediately,
			"zug_faellt_aus" => EventCard::ZugFaelltAus,
			"snack_zone" => EventCard::SnackZone,
			"its_all_in_the_trees" => EventCard::ItsAllInTheTrees,
			"bonjour_to_everyone" => EventCard::BonjourToEveryone,
			"no_talk" => EventCard::NoTalk,
			"slovenia_as_a_treat" => EventCard::SloveniaAsATreat,
			_ => panic!("{value} not a valid EventCard ID"),
		}
	}
}

fn generate_event_card_stack() -> Vec<EventCard> {
	let mut rng = thread_rng();
	let mut output: Vec<EventCard> = Vec::new();
	
	output.push(EventCard::GiveMeYourCards);
	output.push(EventCard::HuntedByMenForSport);
	output.push(EventCard::LuxembourgIsGermanyFrance);
	output.push(EventCard::LetsGoToTheBeach);
	output.push(EventCard::ImagineTrains);
	output.push(EventCard::ConsiderVelocity);
	output.push(EventCard::ItsPopsicle);
	output.push(EventCard::HydrateOrDiedrate);
	output.push(EventCard::StealthOutfit);
	output.push(EventCard::CardinalDirectionsAndVibes);
	output.push(EventCard::Pizzazz);
	output.push(EventCard::RatMode);
	output.push(EventCard::BingBong);
	output.push(EventCard::LeaveCountryImmediately);
	output.push(EventCard::ZugFaelltAus);
	output.push(EventCard::SnackZone);
	output.push(EventCard::ItsAllInTheTrees);
	output.push(EventCard::BonjourToEveryone);
	output.push(EventCard::NoTalk);
	output.push(EventCard::SloveniaAsATreat);

	output.shuffle(&mut rng);

	return output;
}