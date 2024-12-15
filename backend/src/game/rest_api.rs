use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use crate::rest_api::AppState;
use serde::{Deserialize, Serialize};
use super::*;

#[derive(Deserialize, Debug, Clone)]
struct JoinGamePostBody {
  display_name: String,
}

#[post("/api/v1/games")]
pub async fn create_game(data: web::Data<AppState>, req: HttpRequest, body: web::Json<JoinGamePostBody>) -> impl Responder {
	let game = Game::create(body.display_name.clone());

	match data.games.try_lock() {
		Ok(mut games) => {
			let game_id = game.id;
			let invite_code = game.invite_code.clone();

			games.insert(game_id.clone(), game);
			return HttpResponse::Ok().body(format!("{{\"game_id\":\"{game_id}\",\"invite_code\":\"{invite_code}\"}}"));
		},
		Err(e) => return HttpResponse::InternalServerError().body(format!("{{\"error\":\"{e}\"}}")),
	}
}

#[post("/api/v1/invites/{invite_code}/join")]
pub async fn join_game(data: web::Data<AppState>, req: HttpRequest, body: web::Json<JoinGamePostBody>, invite_code: web::Path<String>) -> impl Responder {
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

#[get("/api/v1/games/{game_id}/current_state")]
pub async fn get_current_state(data: web::Data<AppState>, req: HttpRequest, game_id: web::Path<Uuid>) -> impl Responder {
	match data.games.try_lock() {
		Ok(games) => {
			match games.get(&game_id) {
				Some(game) => {
					let current_state_json = match game.state {
						GameState::Lobby => serde_json::to_string(&LobbyGameState {players: game.players.iter().map(|x| x.display_name.clone()).collect()}),
						GameState::InProgress => todo!(),
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