# create game
POST /api/v1/games
body:
```json
{
	"display_name": "TheTxT"
}
```

returns game_id (UUIDv4) + invite_code

```json
{
	"game_id": "2238db88-27d4-4a05-98bc-bd973934b83d",
	"invite_code": "012-345",
}
```

# join game
POST /api/v1/invites/{invite_code}/join
body:
```json
{
	"display_name": "TheTxT"
}
```

returns player_id (UUIDv4) and game_id (UUIDv4)
the game_id is used for all further interaction with the current game
the player_id is unique to the game and will identify the player. not to be shared with the other players as it gets used for authentication during the game
```json
{
	"game_id": "2238db88-27d4-4a05-98bc-bd973934b83d",
	"player_id": "59628524-5c28-4c7e-890f-20bba691853e",
}
```

# start game
POST /api/v1/games/{game_id}/start

used to start the game, can only be called by the host

# get current game state
GET /api/v1/games/{game_id}/current_state

response depends on the current phase of the game (lobby, in_progress, finished)

lobby:
```json
{
	"players": ["ExxPlore, Leon, TheTxT"],
}
```

in_progress:
```json
{
	"runner": "Leon", //the runner for the game, the chasers are all players that are not the runner
	"destination": "Dublin", //only sent to the runner
	"current_turn": "ExxPlore",
	"coins_runner": 2,
	"coins_chasers": 6,
	"your_timetable_cards": ["low_speed", "low_speed", "high_speed", "plane", "joker"],
	"chaser_timetable_cards": {
		"ExxPlore": ["high_speed", "high_speed", "high_speed", "plane", "joker"],
		"TheTxT": ["low_speed", "low_speed", "high_speed", "plane", "joker"],
	},
	"last_used_timetable_card": "low_speed",
	"dice_result": 4, //set only if a dice was rolled last turn
	"event_card_bought": true, //set only to true if an event card was bought last turn
}
```

finished:
```json
{
	"winning_team": "chasers",
	"win_condition": "runner_caught", //runner_caught, timetable_cards_ran_out, reached_destination
	"runner_path": ["nancy", "paris", "london"],
}
```

# make move
POST /api/v1/games/{game_id}/make_move
body:
```json
{
	"player_id" "59628524-5c28-4c7e-890f-20bba691853e",
	"next_location": "copenhagen", //the id of the spot the player moves to
	"use_card": "joker", //the type of card the player used to make the turn
	"buy_event_card": true, //indicates that the player buys an event card
	"use_event_card": "some_card_id", //event card that gets used for the turn
	"buy_powerup": "power_up_id",
	"use_powerup": "power_up_id",
	"throw_timetable_cards_away": ["high_speed", "high_speed"], //max of two per round 
	"finish_move": true, //indicates that the player has finished their turn
}
```

success response (200):
```json
{
	"coins_received": 2, //set if next_location was a coin field
	"event_card_received": "some_other_card_id", //set if player purchased an event card
	"event_card_bought": true, //set true if an event card was already purchased during the current turn, cant buy multiple in a single turn
	"runner_caught": false, //will be true when the hider was caught, gets only sent when finish_move was true
	"timetable_cards_received": ["high_speed", "high_speed"], //may get up to two timetable cards 
}
```

illegal move (400):
```json
{
	"error_id": "not_enough_coins",
	"error_message": "you don't have enough coins to buy an event card",
}
```

can be called multiple times for a single move, because you can buy an event card and use it in the same turn
