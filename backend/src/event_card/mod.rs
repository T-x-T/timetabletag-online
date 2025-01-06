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

impl std::convert::From<String> for EventCard {
	fn from(value: String) -> Self {
		match value.as_str() {
			"give_me_your_cards" => EventCard::GiveMeYourCards,
			"hunted_by_men_for_sport" => EventCard::HuntedByMenForSport,
			"luxembourg_is_germany_france" => EventCard::LuxembourgIsGermanyFrance,
			"lets_go_to_the_beach" => EventCard::LetsGoToTheBeach,
			"imagine_if_trains" => EventCard::ImagineTrains,
			"consider_velocity" => EventCard::ConsiderVelocity,
			"its_popsicle" => EventCard::ItsPopsicle,
			"hydrate_or_diedrate" => EventCard::HydrateOrDiedrate,
			"stealth_outfit" => EventCard::StealthOutfit,
			"cardinal_directions_and_vibes" => EventCard::CardinalDirectionsAndVibes,
			"pizzazz" => EventCard::Pizzazz,
			"rat_mode" => EventCard::RatMode,
			"bing_bong" => EventCard::BingBong,
			"leave_country_immediately" => EventCard::LeaveCountryImmediately,
			"zug_faellt_aus" => EventCard::ZugFaelltAus,
			"snack_zone" => EventCard::SnackZone,
			"its_all_in_the_trees" => EventCard::ItsAllInTheTrees,
			"bonjour_to_everyone" => EventCard::BonjourToEveryone,
			"no_talk" => EventCard::NoTalk,
			"slovenia_as_a_treat" => EventCard::SloveniaAsATreat,
			_ => panic!("{value} not a valid EventCard ID"),
		}
	}
}

pub fn generate_event_card_stack() -> Vec<EventCard> {
	let mut rng = thread_rng();
	let mut output: Vec<EventCard> = Vec::new();
	
	output.push(EventCard::GiveMeYourCards);
	output.push(EventCard::HuntedByMenForSport);
	output.push(EventCard::LuxembourgIsGermanyFrance);
	output.push(EventCard::LetsGoToTheBeach);
	output.push(EventCard::ImagineTrains);
	output.push(EventCard::ConsiderVelocity);
	output.push(EventCard::ItsPopsicle);
	output.push(EventCard::HydrateOrDiedrate);
	output.push(EventCard::StealthOutfit);
	output.push(EventCard::CardinalDirectionsAndVibes);
	output.push(EventCard::Pizzazz);
	output.push(EventCard::RatMode);
	output.push(EventCard::BingBong);
	output.push(EventCard::LeaveCountryImmediately);
	output.push(EventCard::ZugFaelltAus);
	output.push(EventCard::SnackZone);
	output.push(EventCard::ItsAllInTheTrees);
	output.push(EventCard::BonjourToEveryone);
	output.push(EventCard::NoTalk);
	output.push(EventCard::SloveniaAsATreat);

	output.shuffle(&mut rng);

	return output;
}