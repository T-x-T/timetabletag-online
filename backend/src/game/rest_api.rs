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
	let game = Game::create(body.display_name.clone());

	match data.games.try_lock() {
		Ok(mut games) => {
			let game_id = game.id;
			let invite_code = game.invite_code.clone();
			let player_id = game.host;

			games.insert(game_id.clone(), game);
			return HttpResponse::Ok().body(format!("{{\"game_id\":\"{game_id}\",\"invite_code\":\"{invite_code}\",\"player_id\":\"{player_id}\"}}"));
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[post("/api/v1/invites/{invite_code}/join")]
pub async fn join_game(data: web::Data<AppState>, body: web::Json<JoinGamePostBody>, invite_code: web::Path<String>) -> impl Responder {
	match data.games.try_lock() {
		Ok(mut games) => {
			match games.iter().filter(|x| x.1.invite_code == invite_code.clone()).map(|x| x.1).next() {
				Some(game) => {
					let game_id = game.id;

					let game = games.get_mut(&game_id).unwrap();
					match game.join(body.display_name.clone()) {
						Ok(player_id) => return HttpResponse::Ok().body(format!("{{\"game_id\":\"{game_id}\",\"player_id\":\"{player_id}\"}}")),
						Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
					}
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
					let current_state_json = match game.state {
						GameState::Lobby => serde_json::to_string(&LobbyGameState {players: game.players.iter().map(|x| x.display_name.clone()).collect()}),
						GameState::InProgress => serde_json::to_string(&InProgressGameState {
							runner: game.runner.clone().unwrap().display_name,
							destination: if query.player_id.is_some_and(|x| x == game.runner.clone().unwrap().id) {Some(game.destination.clone())} else {None},
							current_turn: game.current_turn.clone().unwrap().display_name,
							coins_runner: game.coins_runner,
							coins_chasers: game.coins_chasers,
							your_timetable_cards: game.timetable_cards.get(&query.player_id.unwrap_or_default()).unwrap_or(&Vec::new()).clone().into_iter().map(|x| x.to_string()).collect(),
							chaser_timetable_cards: game.timetable_cards.clone().into_iter().filter(|x| x.0 != game.runner.clone().unwrap().id).map(|x| (game.players.iter().filter(|y| y.id == x.0).next().unwrap().display_name.clone(), x.1.into_iter().map(|x| x.to_string()).collect())).collect(),
							last_used_timetable_card: if game.last_used_timetable_card.is_some() {game.last_used_timetable_card.clone().unwrap().to_string()} else {String::new()},
							dice_result: game.dice_result,
							event_card_bought: game.event_card_bought,
						}),
						GameState::Finished => todo!(),
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
					match game.start(body.player_id) {
						Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
						Err(e) => return HttpResponse::from_error(e),
					}
				},
				None => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"no game with id {game_id} found\"}}")),
			}
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[post("/api/v1/games/{game_id}/make_move")]
pub async fn make_move(data: web::Data<AppState>, game_id: web::Path<Uuid>, body: web::Json<super::Move>) -> impl Responder {
	match data.games.try_lock() {
		Ok(mut games) => {
			match games.get_mut(&game_id) {
				Some(game) => {
					match game.make_move(body.into_inner()) {
						Ok(res) => return HttpResponse::Ok().body(serde_json::to_string(&res).unwrap()),
						Err(e) => return HttpResponse::from_error(e),
					}
				},
				None => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"no game with id {game_id} found\"}}")),
			}
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}