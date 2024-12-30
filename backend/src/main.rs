mod game;
mod timetable_card;
mod event_card;
mod location;
mod rest_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  rest_api::initialize_webserver().await?;
  println!("Hello, world!");

  return Ok(());
}

#[derive(Debug, Clone)]
pub enum CustomError {
  LobbyFull,
  LobbyNotFullEnough,
  InvalidGameState,
  ActionNotAllowed,
  NotYourTurn,
}

impl std::fmt::Display for CustomError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return match self {
      CustomError::LobbyFull => write!(f, "lobby is already full, a maximum of four players can play at a time"),
      CustomError::LobbyNotFullEnough => write!(f, "not enough players to start game, a minimum of two players are required to play"),
      CustomError::InvalidGameState => write!(f, "the current game isn't in a state where this operation is allowed"),
      CustomError::ActionNotAllowed => write!(f, "you are not allowed to do what you just tried to do"),
      CustomError::NotYourTurn => write!(f, "it's not your turn"),
    }
  }
}

impl std::error::Error for CustomError {

}