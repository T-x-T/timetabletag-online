#[cfg(test)]
mod test;

use std::fmt::Display;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
pub enum EventCard {
	GiveMeYourCards,
	HuntedByMenForSport,
	LuxembourgIsGermanyFrance,
	LetsGoToTheBeach,
	ImagineTrains,
	ConsiderVelocity,
	ItsPopsicle,
	HydrateOrDiedrate,
	StealthOutfit,
	CardinalDirectionsAndVibes,
	Pizzazz,
	RatMode,
	BingBong,
	LeaveCountryImmediately,
	ZugFaelltAus,
	SnackZone,
	ItsAllInTheTrees,
	BonjourToEveryone,
	NoTalk,
	SloveniaAsATreat,
}

impl Display for EventCard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EventCard::GiveMeYourCards => write!(f, "give_me_your_cards"),
			EventCard::HuntedByMenForSport => write!(f, "hunted_by_men_for_sport"),
			EventCard::LuxembourgIsGermanyFrance => write!(f, "luxembourg_is_germany_france"),
			EventCard::LetsGoToTheBeach => write!(f, "lets_go_to_the_beach"),
			EventCard::ImagineTrains => write!(f, "imagine_if_trains"),
			EventCard::ConsiderVelocity => write!(f, "consider_velocity"),
			EventCard::ItsPopsicle => write!(f, "its_popsicle"),
			EventCard::HydrateOrDiedrate => write!(f, "hydrate_or_diedrate"),
			EventCard::StealthOutfit => write!(f, "stealth_outfit"),
			EventCard::CardinalDirectionsAndVibes => write!(f, "cardinal_directions_and_vibes"),
			EventCard::Pizzazz => write!(f, "pizzazz"),
			EventCard::RatMode => write!(f, "rat_mode"),
			EventCard::BingBong => write!(f, "bing_bong"),
			EventCard::LeaveCountryImmediately => write!(f, "leave_country_immediately"),
			EventCard::ZugFaelltAus => write!(f, "zug_faellt_aus"),
			EventCard::SnackZone => write!(f, "snack_zone"),
			EventCard::ItsAllInTheTrees => write!(f, "its_all_in_the_trees"),
			EventCard::BonjourToEveryone => write!(f, "bonjour_to_everyone"),
			EventCard::NoTalk => write!(f, "no_talk"),
			EventCard::SloveniaAsATreat => write!(f, "slovenia_as_a_treat"),
		}
	}
}

impl std::convert::TryFrom<String> for EventCard {
	type Error = String;
	fn try_from(value: String) -> Result<EventCard, String> {
		match value.as_str() {
			"give_me_your_cards" => Ok(EventCard::GiveMeYourCards),
			"hunted_by_men_for_sport" => Ok(EventCard::HuntedByMenForSport),
			"luxembourg_is_germany_france" => Ok(EventCard::LuxembourgIsGermanyFrance),
			"lets_go_to_the_beach" => Ok(EventCard::LetsGoToTheBeach),
			"imagine_if_trains" => Ok(EventCard::ImagineTrains),
			"consider_velocity" => Ok(EventCard::ConsiderVelocity),
			"its_popsicle" => Ok(EventCard::ItsPopsicle),
			"hydrate_or_diedrate" => Ok(EventCard::HydrateOrDiedrate),
			"stealth_outfit" => Ok(EventCard::StealthOutfit),
			"cardinal_directions_and_vibes" => Ok(EventCard::CardinalDirectionsAndVibes),
			"pizzazz" => Ok(EventCard::Pizzazz),
			"rat_mode" => Ok(EventCard::RatMode),
			"bing_bong" => Ok(EventCard::BingBong),
			"leave_country_immediately" => Ok(EventCard::LeaveCountryImmediately),
			"zug_faellt_aus" => Ok(EventCard::ZugFaelltAus),
			"snack_zone" => Ok(EventCard::SnackZone),
			"its_all_in_the_trees" => Ok(EventCard::ItsAllInTheTrees),
			"bonjour_to_everyone" => Ok(EventCard::BonjourToEveryone),
			"no_talk" => Ok(EventCard::NoTalk),
			"slovenia_as_a_treat" => Ok(EventCard::SloveniaAsATreat),
			_ => Err(format!("{value} not a valid EventCard ID")),
		}
	}
}

pub fn generate_event_card_stack() -> Vec<EventCard> {
	let mut rng = thread_rng();
	let mut output: Vec<EventCard> = vec![
		EventCard::GiveMeYourCards,
		EventCard::HuntedByMenForSport,
		EventCard::LuxembourgIsGermanyFrance,
		EventCard::LetsGoToTheBeach,
		EventCard::ImagineTrains,
		EventCard::ConsiderVelocity,
		EventCard::ItsPopsicle,
		EventCard::HydrateOrDiedrate,
		EventCard::StealthOutfit,
		EventCard::CardinalDirectionsAndVibes,
		EventCard::Pizzazz,
		EventCard::RatMode,
		EventCard::BingBong,
		EventCard::LeaveCountryImmediately,
		EventCard::ZugFaelltAus,
		EventCard::SnackZone,
		EventCard::ItsAllInTheTrees,
		EventCard::BonjourToEveryone,
		EventCard::NoTalk,
		EventCard::SloveniaAsATreat,
	];

	output.shuffle(&mut rng);

	return output;
}