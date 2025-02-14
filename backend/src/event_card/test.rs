use super::*;
use std::collections::BTreeMap;

mod generate_event_card_stack {
	use super::*;

	#[test]
	fn is_random_enough() {
		let n = 5_000;

		//check the first
		let mut output: BTreeMap<EventCard, usize> = BTreeMap::new();
		for _ in 0..n {
			output.entry(generate_event_card_stack().first().unwrap().clone()).and_modify(|x| *x += 1).or_insert(1);
		}

		assert!(*output.get(&EventCard::BingBong).unwrap() as f64 / n as f64 > 0.03);
		assert!((*output.get(&EventCard::BingBong).unwrap() as f64 / n as f64) < 0.07);
		
		//and again the last
		let mut output: BTreeMap<EventCard, usize> = BTreeMap::new();
		for _ in 0..n {
			output.entry(generate_event_card_stack().pop().unwrap().clone()).and_modify(|x| *x += 1).or_insert(1);
		}

		assert!(*output.get(&EventCard::BingBong).unwrap() as f64 / n as f64 > 0.03);
		assert!((*output.get(&EventCard::BingBong).unwrap() as f64 / n as f64) < 0.07);

		//ok that should be good enough I guess
	}

	#[test]
	fn card_stack_contains_right_amount_of_each() {
		let res = generate_event_card_stack();
		
		assert_eq!(res.len(), 20);
		assert_eq!(res.iter().filter(|x| **x == EventCard::GiveMeYourCards).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::HuntedByMenForSport).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::LuxembourgIsGermanyFrance).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::LetsGoToTheBeach).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::ImagineTrains).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::ConsiderVelocity).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::ItsPopsicle).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::HydrateOrDiedrate).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::StealthOutfit).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::CardinalDirectionsAndVibes).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::Pizzazz).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::RatMode).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::BingBong).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::LeaveCountryImmediately).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::ZugFaelltAus).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::SnackZone).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::ItsAllInTheTrees).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::BonjourToEveryone).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::NoTalk).count(), 1);
		assert_eq!(res.iter().filter(|x| **x == EventCard::SloveniaAsATreat).count(), 1);
	}
}