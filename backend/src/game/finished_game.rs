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