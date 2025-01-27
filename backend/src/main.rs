mod game;
mod timetable_card;
mod event_card;
mod location;
mod rest_api;
mod powerup;

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
  InvalidNextLocation,
  MissingTimetableCard,
  AlreadyMoved,
  NotEnoughCoins,
  EventCardNoLocationSent,
  EventCardAlreadyBought,
  NotAnEventField,
  EventCardStackEmpty,
  EventCardNotOnYourHand,
  YoureCurrentlyHuntedByMenForSport,
  YouMustGoToGermanyOrFrance,
  YouMustGoNorth,
  YouAreCurrentlyInRatMode,
}

impl std::fmt::Display for CustomError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    return match self {
      CustomError::LobbyFull => write!(f, "lobby is already full, a maximum of four players can play at a time"),
      CustomError::LobbyNotFullEnough => write!(f, "not enough players to start game, a minimum of two players are required to play"),
      CustomError::InvalidGameState => write!(f, "the current game isn't in a state where this operation is allowed"),
      CustomError::ActionNotAllowed => write!(f, "you are not allowed to do what you just tried to do"),
      CustomError::NotYourTurn => write!(f, "it's not your turn"),
      CustomError::InvalidNextLocation => write!(f, "you actually can't get to the chosen next location"),
      CustomError::MissingTimetableCard => write!(f, "you don't have the card you're trying to play"),
      CustomError::AlreadyMoved => write!(f, "you already moved in your current turn"),
      CustomError::NotEnoughCoins => write!(f, "you don't have enough coins"),
      CustomError::EventCardNoLocationSent => write!(f, "you need to send a new location before buying an event card"),
      CustomError::EventCardAlreadyBought => write!(f, "you already bought an event card in this turn"),
      CustomError::NotAnEventField => write!(f, "you can only buy an event card when you're on an event spot"),
      CustomError::EventCardStackEmpty => write!(f, "there aren't any event cards in the stack anymore. Congratulations!"),
      CustomError::EventCardNotOnYourHand => write!(f, "you don't have the event card you're trying to play on your hand."),
      CustomError::YoureCurrentlyHuntedByMenForSport => write!(f, "you're currently hunted by men for sport. You are very scared and must use your fastest method of transport"),
      CustomError::YouMustGoToGermanyOrFrance => write!(f, "what is luxembourg if not germany-france? You must go to either germany or france in your current turn!"),
      CustomError::YouMustGoNorth => write!(f, "you're currently navigating on cardinal directions and vibes and thus must go north!"),
      CustomError::YouAreCurrentlyInRatMode => write!(f, "you're currently in rat mode! You have to use the slowest possible transport method.")
    }
  }
}

impl std::error::Error for CustomError {

}