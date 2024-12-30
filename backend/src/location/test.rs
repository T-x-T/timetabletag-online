use super::*;

mod get_low_speed_connections {
	use super::*;

	#[test]
	fn all_low_speed_connections_fit() {
		for loc in Location::get_iter() {
			let low_speed_connections = loc.get_low_speed_connections();

			for connection in low_speed_connections {
				println!("comparing {loc:?} to {connection:?}");
				assert!(connection.get_low_speed_connections().contains(&loc));
			}
		}
	}
}