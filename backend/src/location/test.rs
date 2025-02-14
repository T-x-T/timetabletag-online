use super::*;
#[test]
fn all_low_speed_connections_fit() {
	for loc in Location::get_iter() {
		let low_speed_connections = loc.get_low_speed_connections();

		for connection in low_speed_connections {
			println!("comparing low speed connection from {loc:?} to {connection:?}");
			assert!(connection.get_low_speed_connections().contains(&loc));
		}
	}
}

#[test]
fn all_high_speed_connections_fit() {
	for loc in Location::get_iter() {
		let high_speed_connections = loc.get_high_speed_connections();

		for connection in high_speed_connections {
			println!("comparing high speed connection from {loc:?} to {connection:?}");
			assert!(connection.get_high_speed_connections().contains(&loc));
		}
	}
}

#[test]
fn all_plane_connections_fit() {
	for loc in Location::get_iter() {
		let plane_connections = loc.get_plane_connections();

		for connection in plane_connections {
			println!("comparing plane speed connection from {loc:?} to {connection:?}");
			assert!(connection.get_plane_connections().contains(&loc));
		}
	}
}

#[test]
fn all_north_connections_actually_exist() {
	for loc in Location::get_iter() {
		let north_connections = loc.get_north_connections();

		for connection in north_connections {
			println!("checking if north connection {connection:?} actually is a connection of {loc:?}");
			assert!(connection.get_joker_connections().contains(&loc));
		}
	}
}

#[test]
fn all_north_connections_are_unique() {
	for loc in Location::get_iter() {
		let north_connections = loc.get_north_connections();

		for connection in north_connections {
			println!("checking if north connection {connection:?} is a unique north connection of {loc:?}");
			assert!(!connection.get_north_connections().contains(&loc));
		}
	}
}