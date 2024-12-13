use super::*;

mod create {
	use super::*;

	#[test]
	fn properly_adds_first_player() {
		let display_name = "test".to_string();
		let res = Game::create(display_name.clone());

		assert_eq!(res.players.clone().first().unwrap().display_name, display_name);
		assert_eq!(res.players.clone().first().unwrap().id, res.host);
	}

	#[test]
	fn destination_random_enough() {
		let n = 2_500; //passed 10k test runs with 2500, lower might be flakey

		let mut output: BTreeMap<String, usize> = BTreeMap::new();
		for _ in 0..n {
			let destination = Game::create(String::new()).destination;
			output.entry(destination).and_modify(|x| *x += 1).or_insert(1);
		}

		assert_eq!(output.len(), 5);
		assert!(output.iter().map(|x| *x.1).min().unwrap() > n / 6);
		assert!(output.iter().map(|x| *x.1).max().unwrap() < n / 4);
	}

	
}