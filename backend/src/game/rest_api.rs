use actix_web::{get, post, put, delete, web, HttpResponse, HttpRequest, Responder};
use crate::rest_api::AppState;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct CreateGamePostBody {
  display_name: String,
}

#[post("/api/v1/games")]
pub async fn create_game(data: web::Data<AppState>, req: HttpRequest, body: web::Json<CreateGamePostBody>) -> impl Responder {
	let game = super::Game::create(body.display_name.clone());

	let game_id = game.id;
	let invite_code = "123-456".to_string();

	return HttpResponse::Ok().body(format!("{{\"game_id\":\"{game_id}\",\"invite_code\":\"{invite_code}\"}}"));
}