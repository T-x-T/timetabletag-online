use super::*;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FinishedGame {
	pub id: GameId,
	pub host: PlayerId,
	pub runner: PlayerId,
	pub players: Vec<Player>,
	pub destination: Location,
	pub coins_runner: usize,
	pub coins_chasers: usize,
	pub winning_team: Team,
	pub win_condition: WinCondition,
	pub runner_path: Vec<Location>,
}

impl FinishedGame {
	pub fn from_in_progress_game(in_progress_game: &InProgressGame, winning_team: Team, win_condition: WinCondition) -> FinishedGame {
		return FinishedGame {
			id: in_progress_game.id,
			host: in_progress_game.host,
			runner: in_progress_game.runner,
			players: in_progress_game.players.clone(),
			destination: in_progress_game.destination,
			coins_runner: in_progress_game.coins_runner,
			coins_chasers: in_progress_game.coins_chasers,
			winning_team,
			win_condition,
			runner_path: in_progress_game.runner_path.clone(), 
		};
	}
}