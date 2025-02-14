use super::*;
use std::collections::BTreeMap;

mod generate_timetable_card_stack {
	use super::*;

#[test]
	fn is_random_enough() {
		let n = 1_000;

		//check the first
		let mut output: BTreeMap<TimetableCard, usize> = BTreeMap::new();
		for _ in 0..n {
			output.entry(generate_timetable_card_stack().first().unwrap().clone()).and_modify(|x| *x += 1).or_insert(1);
		}

		assert!(*output.get(&TimetableCard::LowSpeed).unwrap() as f64 / n as f64 > 0.4);
		assert!((*output.get(&TimetableCard::LowSpeed).unwrap() as f64 / n as f64) < 0.6);

		assert!(*output.get(&TimetableCard::HighSpeed).unwrap() as f64 / n as f64 > 0.25);
		assert!((*output.get(&TimetableCard::HighSpeed).unwrap() as f64 / n as f64) < 0.36);

		assert!(*output.get(&TimetableCard::Plane).unwrap() as f64 / n as f64 > 0.1);
		assert!((*output.get(&TimetableCard::Plane).unwrap() as f64 / n as f64) < 0.24);

		assert!(*output.get(&TimetableCard::Joker).unwrap() as f64 / n as f64 > 0.01);
		assert!((*output.get(&TimetableCard::Joker).unwrap() as f64 / n as f64) < 0.1);
		
		//and again the last
		let mut output: BTreeMap<TimetableCard, usize> = BTreeMap::new();
		for _ in 0..n {
			output.entry(generate_timetable_card_stack().pop().unwrap().clone()).and_modify(|x| *x += 1).or_insert(1);
		}

		assert!(*output.get(&TimetableCard::LowSpeed).unwrap() as f64 / n as f64 > 0.4);
		assert!((*output.get(&TimetableCard::LowSpeed).unwrap() as f64 / n as f64) < 0.6);

		assert!(*output.get(&TimetableCard::HighSpeed).unwrap() as f64 / n as f64 > 0.25);
		assert!((*output.get(&TimetableCard::HighSpeed).unwrap() as f64 / n as f64) < 0.36);

		assert!(*output.get(&TimetableCard::Plane).unwrap() as f64 / n as f64 > 0.1);
		assert!((*output.get(&TimetableCard::Plane).unwrap() as f64 / n as f64) < 0.24);

		assert!(*output.get(&TimetableCard::Joker).unwrap() as f64 / n as f64 > 0.01);
		assert!((*output.get(&TimetableCard::Joker).unwrap() as f64 / n as f64) < 0.1);

		//ok that should be good enough I guess
	}

	#[test]
	fn card_stack_contains_right_amount_of_each() {
		let res = generate_timetable_card_stack();
		
		assert_eq!(res.len(), 100);
		assert_eq!(res.iter().filter(|x| **x == TimetableCard::LowSpeed).count(), 50);
		assert_eq!(res.iter().filter(|x| **x == TimetableCard::HighSpeed).count(), 30);
		assert_eq!(res.iter().filter(|x| **x == TimetableCard::Plane).count(), 16);
		assert_eq!(res.iter().filter(|x| **x == TimetableCard::Joker).count(), 4);
	}
}