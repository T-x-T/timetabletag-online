use actix_web::{get, post, web, HttpResponse, Responder};
use crate::rest_api::AppState;
use serde::{Deserialize, Serialize};
use super::*;

#[derive(Deserialize, Debug, Clone)]
struct JoinGamePostBody {
  display_name: String,
}

#[post("/api/v1/games")]
pub async fn create_game(data: web::Data<AppState>, body: web::Json<JoinGamePostBody>) -> impl Responder {
	let game = Lobby::create(body.display_name.clone());

	match data.games.try_lock() {
		Ok(mut games) => {
			let game_id = game.id;
			let invite_code = game.invite_code.clone();
			let player_id = game.host;

			games.insert(game_id.clone(), Game::Lobby(game));
			return HttpResponse::Ok().body(format!("{{\"game_id\":\"{game_id}\",\"invite_code\":\"{invite_code}\",\"player_id\":\"{player_id}\"}}"));
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[post("/api/v1/invites/{invite_code}/join")]
pub async fn join_game(data: web::Data<AppState>, body: web::Json<JoinGamePostBody>, invite_code: web::Path<String>) -> impl Responder {
	match data.games.try_lock() {
		Ok(mut games) => {
			match games.iter().filter(|x| {
				match x.1 {
					Game::Lobby(lobby) => lobby.invite_code == invite_code.clone(),
					Game::InProgress(_) => false,
					Game::Finished(_) => false,
				}
			}).map(|x| x.1).next() {
				Some(game) => {
					match game {
						Game::Lobby(lobby) => {
							let game_id = lobby.id;

							let game = games.get_mut(&game_id).unwrap();
							match game {
								Game::Lobby(lobby) => {
									match lobby.join(body.display_name.clone()) {
										Ok(player_id) => return HttpResponse::Ok().body(format!("{{\"game_id\":\"{game_id}\",\"player_id\":\"{player_id}\"}}")),
										Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
									}
								},
								Game::InProgress(_) => return HttpResponse::BadRequest().body("you cant do that while the game is in its current state"),
								Game::Finished(_) => return HttpResponse::BadRequest().body("you cant do that while the game is in its current state"),
							}
						},
						Game::InProgress(_) => return HttpResponse::BadRequest().body("you cant do that while the game is in its current state"),
						Game::Finished(_) => return HttpResponse::BadRequest().body("you cant do that while the game is in its current state"),
					};
				},
				None => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"no game with invite code {invite_code} found\"}}")),
			}
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[derive(Debug, Clone, Serialize)]
struct LobbyGameState {
	players: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct InProgressGameState {
	runner: String,
	destination: Option<String>,
	current_turn: String,
	coins_runner: usize,
	coins_chasers: usize,
	your_timetable_cards: Vec<String>,
	chaser_timetable_cards: BTreeMap<String, Vec<String>>,
	last_used_timetable_card: String,
	dice_result: Option<u8>,
	event_card_bought: bool,
	runner_current_country: String,
	runner_current_location: String,
	runner_destination: String,
	chaser_gets_another_turn: bool,
	chaser_locations: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
struct GetCurrentStateQueryOptions {
	player_id: Option<Uuid>,
}

#[get("/api/v1/games/{game_id}/current_state")]
pub async fn get_current_state(data: web::Data<AppState>, game_id: web::Path<Uuid>, query: web::Query<GetCurrentStateQueryOptions>) -> impl Responder {
	match data.games.try_lock() {
		Ok(games) => {
			match games.get(&game_id) {
				Some(game) => {
					let current_state_json = match game {
						Game::Lobby(game) => serde_json::to_string(&LobbyGameState {players: game.players.iter().map(|x| x.display_name.clone()).collect()}),
						Game::InProgress(game) => serde_json::to_string(&InProgressGameState {
							runner: game.players.iter().filter(|x| x.id == game.runner).next().unwrap().display_name.clone(),
							destination: if query.player_id.is_some_and(|x| x == game.runner) {Some(game.destination.clone().to_string())} else {None},
							current_turn: game.players.iter().filter(|x| x.id == game.current_turn).next().unwrap().display_name.clone(),
							coins_runner: game.coins_runner,
							coins_chasers: game.coins_chasers,
							your_timetable_cards: game.players.iter().find(|x| x.id == query.player_id.unwrap_or_default()).unwrap().timetable_cards.iter().map(|x| x.to_string()).collect(),
							chaser_timetable_cards: game.players.iter().filter(|x| x.id != game.runner).map(|x| (x.display_name.clone(), x.timetable_cards.iter().map(|x| x.to_string()).collect())).collect(),
							last_used_timetable_card: if game.last_used_timetable_card.is_some() {game.last_used_timetable_card.clone().unwrap().to_string()} else {String::new()},
							dice_result: game.dice_result,
							event_card_bought: game.event_card_bought,
							runner_current_country: if game.power_up_status.runner_country.is_some() {game.power_up_status.runner_country.unwrap().to_string()} else {String::default()},
							runner_current_location: if game.power_up_status.runner_location.is_some() {game.power_up_status.runner_location.unwrap().to_string()} else {String::default()},
							runner_destination: if game.power_up_status.runner_destination.is_some() {game.power_up_status.runner_destination.unwrap().to_string()} else {String::default()},
							chaser_gets_another_turn: game.power_up_status.get_another_turn,
							chaser_locations: game.players.iter().filter(|x| x.id != game.runner || !x.stealth_mode_active).map(|x| (x.display_name.clone(), x.current_location.to_string())).collect()
						}),
						Game::Finished(game) => serde_json::to_string(game),
					}.unwrap();
					return HttpResponse::Ok().body(current_state_json);
				},
				None => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"no game with id {game_id} found\"}}")),
			}
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[derive(Deserialize, Debug, Clone)]
struct StartGamePostBody {
  player_id: Uuid,
}
#[post("/api/v1/games/{game_id}/start")]
pub async fn start_game(data: web::Data<AppState>, game_id: web::Path<Uuid>, body: web::Json<StartGamePostBody>) -> impl Responder {
	match data.games.try_lock() {
		Ok(mut games) => {
			match games.get_mut(&game_id) {
				Some(game) => {
					match game {
						Game::Lobby(lobby) => {
							match lobby.start(body.player_id) {
								Ok(in_progress_game) => {
									*game = Game::InProgress(in_progress_game);
									return HttpResponse::Ok().body("")
								},
								Err(e) => return HttpResponse::from_error(e),
						}},
						Game::InProgress(_) => return HttpResponse::BadRequest().body("you cant do that while the game is in its current state"),
						Game::Finished(_) => return HttpResponse::BadRequest().body("you cant do that while the game is in its current state"),
					};
				},
				None => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"no game with id {game_id} found\"}}")),
			}
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[post("/api/v1/games/{game_id}/make_move")]
pub async fn make_move(data: web::Data<AppState>, game_id: web::Path<Uuid>, body: web::Json<crate::game::in_progress_game::Move>) -> impl Responder {
	match data.games.try_lock() {
		Ok(mut games) => {
			match games.get_mut(&game_id) {
				Some(game) => {
					match game {
						Game::Lobby(_) => return HttpResponse::BadRequest().body("you cant do that while the game is in its current state"),
						Game::InProgress(in_progress_game) => {
							match in_progress_game.make_move(body.into_inner()) {
								Ok(res) => {
									if res.finished_game.is_some() {
										*game = Game::Finished(res.clone().finished_game.unwrap());
									}
									return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap())
								},
								Err(e) => return HttpResponse::from_error(e),
							}
						},
						Game::Finished(_) => return HttpResponse::BadRequest().body("you cant do that while the game is in its current state"),
					};
				},
				None => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"no game with id {game_id} found\"}}")),
			}
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}