use serde::{Serialize, Deserialize};
use std::fmt::Display;
use std::io::{self, Write};
use std::collections::BTreeMap;

fn main() {
  println!("Starting timetabletag-online cli client");

  let res: String = ureq::get("http://localhost:4000").call().err().unwrap().to_string();
  if res == "http status: 404" {
    println!("backend is reachable");
  } else {
    panic!("backend isnt reachable!!");
  }

  let game_id: String;
  let player_id: String;

  let create_or_join = input("(c)reate or (j)oin game?: ");
  let display_name = input("Your Name: ");

  if create_or_join.starts_with("c") {
    println!("creating new game...");

    let create_game_res = ureq::post("http://localhost:4000/api/v1/games")
      .send_json(&CreateGamePostBody {display_name: display_name.clone()})
      .unwrap()
      .body_mut()
      .read_json::<CreateGamePostResponse>()
      .unwrap();

    println!("created game: {create_game_res:?}");

    game_id = create_game_res.game_id;
    player_id = create_game_res.player_id;

    let mut start_game = false;
    while !start_game {
      println!("{:?}", get_lobby_game_state(&game_id));
      start_game = input("(s)tart or (w)ait? ").starts_with("s");
    }

    let _ = ureq::post(format!("http://localhost:4000/api/v1/games/{}/start", &game_id))
      .send_json(&StartGamePostBody {player_id: player_id.clone()})
      .unwrap()
      .body_mut()
      .read_to_string()
      .unwrap();
  } else {
    let invite_code = input("Invite code: ");
    
    let join_game_res = ureq::post(format!("http://localhost:4000/api/v1/invites/{invite_code}/join"))
      .send_json(&JoinGamePostBody {display_name: display_name.clone()})
      .unwrap()
      .body_mut()
      .read_json::<JoinGamePostResponse>()
      .unwrap();

    println!("joined game: {join_game_res:?}");

    game_id = join_game_res.game_id;
    player_id = join_game_res.player_id;

    let _ = input("(c)ontinue: ");
  }

  let mut last_game_state = InProgressGameState::default();

  loop {
    let current_state = get_in_progress_game_state(&game_id, &player_id);
    if current_state.is_err() {
      println!("got error while trying to get in_progress_game_state: {}", current_state.err().unwrap());
      std::thread::sleep(std::time::Duration::from_secs(1));
      continue;
    }
    let current_state = current_state.unwrap();
    
    if current_state != last_game_state {
      println!("{current_state:?}");
      println!("{current_state}");
    }
    
    last_game_state = current_state.clone();
    if current_state.current_turn == display_name {
      println!("its your turn!");
      let res = make_move(&game_id, &player_id);
      println!("{res:?}");
      println!("{res}");
    }

    std::thread::sleep(std::time::Duration::from_secs(1));
  }

}


fn input(message: &str) -> String {
  print!("{message}");
  io::stdout().flush().expect("failed to flush stdout");
  let mut response = String::new();
  io::stdin().read_line(&mut response).expect("failed to read line");
  return response.trim().to_string();
}

fn get_lobby_game_state(game_id: &str) -> LobbyGameState {
  return ureq::get(format!("http://localhost:4000/api/v1/games/{game_id}/current_state"))
    .call()
    .unwrap()
    .body_mut()
    .read_json::<LobbyGameState>()
    .unwrap();
}

fn get_in_progress_game_state(game_id: &str, player_id: &str) -> Result<InProgressGameState, String> {
  return match ureq::get(format!("http://localhost:4000/api/v1/games/{game_id}/current_state"))
    .query("player_id", player_id)
    .call() {
      Ok(mut x) => {
        Ok(x.body_mut()
          .read_json::<InProgressGameState>()
          .unwrap())
      },
      Err(ureq::Error::StatusCode(code)) => {
        Err(format!("got error code {code}"))
      },
      Err(_) => panic!("aaaaaaaa"),
    }
}

fn make_move(game_id: &str, player_id: &str) -> String {
  let type_of_move = input("(b)uy/(u)se event card, buy (p)owerup, (t)hrow timetable cards away, (m)ove or (f)inish move? ");

  let move_to_make = match type_of_move.as_str() {
    "b" => {
      MakeMovePostBody {
        player_id: player_id.to_string(),
        buy_event_card: true,
        ..Default::default()
      }
    },
    "u" => {
      let event_card_id = input("event card to play: ");
      MakeMovePostBody {
        player_id: player_id.to_string(),
        use_event_card: Some(event_card_id),
        ..Default::default()
      }
    },
    "p" => {
      let powerup_to_buy = input("powerup to buy: ");
      MakeMovePostBody {
        player_id: player_id.to_string(),
        buy_powerup: Some(powerup_to_buy),
        ..Default::default()
      }
    },
    "t" => {
      let timetable_card_1 = match input("timetable card (l)ow/(h)igh speed, (p)lane or (j)oker: ").as_str() {
        "l" => "low_speed",
        "h" => "high_speed",
        "p" => "plane",
        "j" => "joker",
        _ => "",
      };
      let timetable_card_2 = match input("timetable card (l)ow/(h)igh speed, (p)lane or (j)oker: ").as_str() {
        "l" => "low_speed",
        "h" => "high_speed",
        "p" => "plane",
        "j" => "joker",
        _ => "",
      };
      let mut timetable_cards: Vec<String> = Vec::new();
      if timetable_card_1.len() > 0 {
        timetable_cards.push(timetable_card_1.to_string());
      }
      if timetable_card_2.len() > 0 {
        timetable_cards.push(timetable_card_2.to_string());
      }
      MakeMovePostBody {
        player_id: player_id.to_string(),
        throw_timetable_cards_away: timetable_cards,
        ..Default::default()
      }
    },
    "m" => {
      let timetable_card = match input("timetable card (l)ow/(h)igh speed, (p)lane or (j)oker: ").as_str() {
        "l" => "low_speed",
        "h" => "high_speed",
        "p" => "plane",
        "j" => "joker",
        _ => panic!("invalid card used"),
      };
      let next_location = input("next location: ");
    
      MakeMovePostBody {
        player_id: player_id.to_string(),
        next_location: Some(next_location),
        use_timetable_card: Some(timetable_card.to_string()),
        ..Default::default()
      }
    },
    "f" => {
      MakeMovePostBody {
        player_id: player_id.to_string(),
        finish_move: true,
        ..Default::default()
      }
    },
    _ => panic!("I dont know what that means"),
  };

  println!("{move_to_make:?}");

  let move_result = Into::<ureq::Agent>::into(ureq::Agent::config_builder()
    .http_status_as_error(false)
    .build())
    .post(format!("http://localhost:4000/api/v1/games/{game_id}/make_move"))
    .send_json(&move_to_make)
    .unwrap()
    .body_mut()
    .read_json::<MakeMovePostResponse>()
    .unwrap();

  if move_result.error_id.is_some() {
    return format!("{}: {}", move_result.error_id.unwrap(), move_result.error_message.unwrap());
  } else {
    return format!("runner caught: {}\ncoins received: {}\ntimetable cards received: {}", move_result.runner_caught.unwrap_or_default(), move_result.coins_received.unwrap_or_default(), move_result.timetable_cards_received.unwrap_or_default().join(", "));
  }
}

#[derive(Debug, Serialize)]
struct CreateGamePostBody {
  display_name: String,
}

#[derive(Debug, Deserialize)]
struct CreateGamePostResponse {
  game_id: String,
  invite_code: String,
  player_id: String,
}

#[derive(Debug, Serialize)]
struct JoinGamePostBody {
  display_name: String,
}

#[derive(Debug, Deserialize)]
struct JoinGamePostResponse {
  game_id: String,
  player_id: String,
}

#[derive(Debug, Deserialize)]
struct LobbyGameState {
  players: Vec<String>,
}

#[derive(Debug, Serialize)]
struct StartGamePostBody {
  player_id: String,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
struct InProgressGameState {
  runner: String,
	destination: Option<String>,
	current_turn: String,
	coins_runner: u8,
	coins_chasers: u8,
	your_timetable_cards: Vec<String>,
	chaser_timetable_cards: BTreeMap<String, Vec<String>>,
	last_used_timetable_card: String,
	dice_result: Option<u8>,
	event_card_bought: bool,
	runner_current_country: String,
	runner_destination: String,
	chaser_gets_another_turn: bool,
	chaser_locations: BTreeMap<String, String>,
  your_current_location: String,
}

impl Display for InProgressGameState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    return write!(f, "The runner is: {}\nCurrent turn: {}\nYour timetable cards: {}\nYour location: {}", self.runner, self.current_turn, self.your_timetable_cards.join(", "), self.your_current_location);
  }
}

#[derive(Debug, Default, Serialize)]
struct MakeMovePostBody {
  player_id: String,
	next_location: Option<String>,
	use_timetable_card: Option<String>,
	buy_event_card: bool,
	use_event_card: Option<String>,
	buy_powerup: Option<String>,
	throw_timetable_cards_away: Vec<String>,
	finish_move: bool,
}

#[derive(Debug, Deserialize)]
struct MakeMovePostResponse {
  coins_received: Option<u8>,
  event_card_received: Option<String>,
	event_card_bought: Option<bool>,
	runner_caught: Option<bool>,
	timetable_cards_received: Option<Vec<String>>,
  error_id: Option<String>,
  error_message: Option<String>,
}