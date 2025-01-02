use std::fmt::Display;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
pub enum Location {
	//Ireland
	Dublin,
	Rosslare,
	//UK
	Belfast,
	Cairnryan,
	Glasgow,
	Edinburgh,
	Newcastle,
	York,
	Liverpool,
	Holyhead,
	Fishguard,
	Swansea,
	Birmingham,
	Nottingham,
	Cambridge,
	Oxford,
	Plymouth,
	Bournemouth,
	London,
	//Denmark
	Aalborg,
	Aarhus,
	Esbjerg,
	Copenhagen,
	//Spain
	Bilbao,
	Burgos,
	Pamplona,
	Valladolid,
	Zaragoza,
	Madrid,
	Albacete,
	Valencia,
	Barcelona,
	//Andorra
	Andorra,
	//France
	Calais,
	LeHavre,
	Paris,
	CharlevilleMezieres,
	Brest,
	Rennes,
	Nantes,
	LeMans,
	Orleans,
	Poitiers,
	LaRochelle,
	Limoges,
	Bordeaux,
	ClermontFerrand,
	Toulouse,
	Montpellier,
	Nancy,
	Strasbourg,
	Dijon,
	Lyon,
	Grenoble,
	Marseille,
	Nice,
	//Netherlands
	Groningen,
	Amsterdam,
	TheHague,
	SHertogenbosch,
	//Belgium
	Ghent,
	Antwerp,
	Brussels,
	//Luxembourg
	Luxembourg,
	//Germany
	Kiel,
	Bremen,
	Hamburg,
	Rostock,
	Bielefeld,
	Magdeburg,
	Berlin,
	Cologne,
	Kassel,
	Erfurt,
	Leipzig,
	Dresden,
	Frankfurt,
	Nuremberg,
	Stuttgart,
	Munich,
	//Switzerland
	Basel,
	Zurich,
	Merlischachen,
	Geneva,
	//Austria
	Innsbruck,
	Salzburg,
	Linz,
	Vienna,
	Villach,
	Graz,
	//Italy
	Bolzano,
	Trento,
	Turin,
	Milan,
	Padua,
	Venice,
	Genoa,
	Bologna,
	Pisa,
	Florence,
	SanMarino,
	Perugia,
	Rome,
	//Poland
	Gdansk,
	Szczecin,
	Bydgoszcz,
	Poznan,
	Wroclaw,
	//Czech Republic
	Pilsen,
	Prague,
	Liberec,
	CeskeBudejovice,
	Brno,
	Ostrava,
	//Hungary
	Sopron,
	//Slovenia
	Ljubljana,
	//Croatia
	Rijeka,
	Zagreb,
	Split,
	//Bosnia and Herzegovina
	BanjaLuka,
}

impl Location {
	pub fn get_low_speed_connections(&self) -> Vec<Location> {
		match self {
			Location::Dublin => vec![Location::Belfast, Location::Holyhead, Location::Rosslare],
			Location::Rosslare => vec![Location::Dublin, Location::Fishguard],
			Location::Belfast => vec![Location::Dublin, Location::Cairnryan],
			Location::Cairnryan => vec![Location::Belfast, Location::Glasgow],
			Location::Glasgow => vec![Location::Cairnryan, Location::Liverpool, Location::Edinburgh],
			Location::Edinburgh => vec![Location::Glasgow, Location::Newcastle],
			Location::Newcastle => vec![Location::Edinburgh, Location::York],
			Location::York => vec![Location::Newcastle, Location::Liverpool, Location::Nottingham],
			Location::Liverpool => vec![Location::Glasgow, Location::York, Location::Birmingham, Location::Holyhead],
			Location::Holyhead => vec![Location::Dublin, Location::Liverpool],
			Location::Fishguard => vec![Location::Rosslare, Location::Swansea],
			Location::Swansea => vec![Location::Fishguard, Location::Birmingham],
			Location::Birmingham => vec![Location::Liverpool, Location::Nottingham, Location::London, Location::Oxford, Location::Swansea],
			Location::Nottingham => vec![Location::York, Location::Cambridge, Location::Birmingham],
			Location::Cambridge => vec![Location::Nottingham, Location::London],
			Location::Oxford => vec![Location::Birmingham, Location::London, Location::Bournemouth, Location::Plymouth],
			Location::Plymouth => vec![Location::Oxford, Location::Bournemouth, Location::Brest],
			Location::Bournemouth => vec![Location::Oxford, Location::London, Location::Plymouth],
			Location::London => vec![Location::Cambridge, Location::Calais, Location::Bournemouth, Location::Oxford, Location::Birmingham],
			Location::Aalborg => vec![Location::Aarhus, Location::Esbjerg],
			Location::Aarhus => vec![Location::Aalborg, Location::Copenhagen, Location::Esbjerg],
			Location::Esbjerg => vec![Location::Aalborg, Location::Aarhus, Location::Kiel],
			Location::Copenhagen => vec![Location::Aarhus, Location::Rostock, Location::Kiel],
			Location::Bilbao => vec![Location::Pamplona, Location::Burgos],
			Location::Burgos => vec![Location::Bilbao, Location::Pamplona, Location::Valladolid],
			Location::Pamplona => vec![Location::Bordeaux, Location::Zaragoza, Location::Burgos, Location::Bilbao],
			Location::Valladolid => vec![Location::Burgos, Location::Madrid],
			Location::Zaragoza => vec![Location::Pamplona, Location::Toulouse, Location::Andorra, Location::Barcelona, Location::Valencia, Location::Madrid],
			Location::Madrid => vec![Location::Valladolid, Location::Zaragoza, Location::Albacete],
			Location::Albacete => vec![Location::Madrid, Location::Valencia],
			Location::Valencia => vec![Location::Zaragoza, Location::Barcelona, Location::Albacete],
			Location::Barcelona => vec![Location::Andorra, Location::Valencia, Location::Zaragoza],
			Location::Andorra => vec![Location::Toulouse, Location::Barcelona, Location::Zaragoza],
			Location::Calais => vec![Location::London, Location::Ghent, Location::Paris],
			Location::LeHavre => vec![Location::Paris, Location::Rennes],
			Location::Paris => vec![Location::Calais, Location::Brussels, Location::CharlevilleMezieres, Location::Nancy, Location::Dijon, Location::Orleans, Location::LeMans, Location::LeHavre],
			Location::CharlevilleMezieres => vec![Location::Luxembourg, Location::Nancy, Location::Paris],
			Location::Brest => vec![Location::Plymouth, Location::Rennes, Location::Nantes],
			Location::Rennes => vec![Location::LeHavre, Location::LeMans, Location::Nantes, Location::Brest],
			Location::Nantes => vec![Location::Rennes, Location::LaRochelle, Location::Brest],
			Location::LeMans => vec![Location::Paris, Location::Poitiers, Location::Rennes],
			Location::Orleans => vec![Location::Paris, Location::Limoges, Location::Poitiers],
			Location::Poitiers => vec![Location::LeMans, Location::Orleans, Location::Limoges, Location::LaRochelle],
			Location::LaRochelle => vec![Location::Nantes, Location::Poitiers, Location::Bordeaux],
			Location::Limoges => vec![Location::Poitiers, Location::Orleans, Location::ClermontFerrand, Location::Toulouse, Location::Bordeaux],
			Location::Bordeaux => vec![Location::LaRochelle, Location::Limoges, Location::Toulouse, Location::Pamplona],
			Location::ClermontFerrand => vec![Location::Lyon, Location::Montpellier, Location::Limoges],
			Location::Toulouse => vec![Location::Limoges, Location::Montpellier, Location::Andorra, Location::Zaragoza, Location::Bordeaux],
			Location::Montpellier => vec![Location::ClermontFerrand, Location::Marseille, Location::Toulouse],
			Location::Nancy => vec![Location::Luxembourg, Location::Strasbourg, Location::Frankfurt, Location::Dijon, Location::Paris, Location::CharlevilleMezieres],
			Location::Strasbourg => vec![Location::Stuttgart, Location::Basel, Location::Nancy],
			Location::Dijon => vec![Location::Nancy, Location::Basel, Location::Lyon, Location::Paris],
			Location::Lyon => vec![Location::Dijon, Location::Geneva, Location::Grenoble, Location::ClermontFerrand],
			Location::Grenoble => vec![Location::Geneva, Location::Turin, Location::Marseille, Location::Lyon],
			Location::Marseille => vec![Location::Grenoble, Location::Nice, Location::Montpellier],
			Location::Nice => vec![Location::Genoa, Location::Marseille],
			Location::Groningen => vec![Location::Bremen, Location::Amsterdam],
			Location::Amsterdam => vec![Location::Groningen, Location::TheHague],
			Location::TheHague => vec![Location::Amsterdam, Location::Antwerp],
			Location::SHertogenbosch => vec![Location::Antwerp],
			Location::Ghent => vec![Location::Antwerp, Location::Brussels, Location::Calais],
			Location::Antwerp => vec![Location::TheHague, Location::SHertogenbosch, Location::Brussels, Location::Ghent],
			Location::Brussels => vec![Location::Antwerp, Location::Cologne, Location::Luxembourg, Location::Paris, Location::Ghent],
			Location::Luxembourg => vec![Location::Brussels, Location::Frankfurt, Location::Nancy, Location::CharlevilleMezieres],
			Location::Kiel => vec![Location::Esbjerg, Location::Copenhagen, Location::Hamburg],
			Location::Bremen => vec![Location::Groningen, Location::Hamburg, Location::Bielefeld],
			Location::Hamburg => vec![Location::Kiel, Location::Rostock, Location::Magdeburg, Location::Bremen],
			Location::Rostock => vec![Location::Copenhagen, Location::Berlin, Location::Hamburg],
			Location::Bielefeld => vec![Location::Bremen, Location::Magdeburg, Location::Kassel, Location::Cologne],
			Location::Magdeburg => vec![Location::Hamburg, Location::Berlin, Location::Erfurt, Location::Bielefeld],
			Location::Berlin => vec![Location::Rostock, Location::Szczecin, Location::Poznan, Location::Dresden, Location::Magdeburg],
			Location::Cologne => vec![Location::Bielefeld, Location::Frankfurt, Location::Brussels],
			Location::Kassel => vec![Location::Bielefeld, Location::Erfurt, Location::Frankfurt],
			Location::Erfurt => vec![Location::Magdeburg, Location::Leipzig, Location::Nuremberg, Location::Frankfurt, Location::Kassel],
			Location::Leipzig => vec![Location::Dresden, Location::Erfurt],
			Location::Dresden => vec![Location::Berlin, Location::Prague, Location::Leipzig],
			Location::Frankfurt => vec![Location::Kassel, Location::Erfurt, Location::Stuttgart, Location::Nancy, Location::Luxembourg, Location::Cologne],
			Location::Nuremberg => vec![Location::Erfurt, Location::Pilsen, Location::Munich, Location::Stuttgart],
			Location::Stuttgart => vec![Location::Frankfurt, Location::Nuremberg, Location::Munich, Location::Strasbourg],
			Location::Munich => vec![Location::Nuremberg, Location::Innsbruck, Location::Stuttgart],
			Location::Basel => vec![Location::Strasbourg, Location::Zurich, Location::Merlischachen, Location::Dijon],
			Location::Zurich => vec![Location::Innsbruck, Location::Merlischachen, Location::Basel, Location::Milan],
			Location::Merlischachen => vec![Location::Zurich, Location::Basel],
			Location::Geneva => vec![Location::Lyon, Location::Grenoble],
			Location::Innsbruck => vec![Location::Munich, Location::Salzburg, Location::Bolzano, Location::Zurich],
			Location::Salzburg => vec![Location::Linz, Location::Villach, Location::Innsbruck],
			Location::Linz => vec![Location::CeskeBudejovice, Location::Vienna, Location::Salzburg],
			Location::Vienna => vec![Location::Brno, Location::Sopron, Location::Linz],
			Location::Villach => vec![Location::Salzburg, Location::Graz, Location::Ljubljana, Location::Venice],
			Location::Graz => vec![Location::Sopron, Location::Villach],
			Location::Bolzano => vec![Location::Innsbruck, Location::Trento],
			Location::Trento => vec![Location::Bolzano, Location::Padua],
			Location::Turin => vec![Location::Milan, Location::Genoa, Location::Grenoble],
			Location::Milan => vec![Location::Zurich, Location::Padua, Location::Bologna, Location::Genoa, Location::Turin],
			Location::Padua => vec![Location::Trento, Location::Venice, Location::Bologna, Location::Milan],
			Location::Venice => vec![Location::Villach, Location::Ljubljana, Location::Padua],
			Location::Genoa => vec![Location::Milan, Location::Pisa, Location::Nice, Location::Turin],
			Location::Bologna => vec![Location::Padua, Location::SanMarino, Location::Florence, Location::Milan],
			Location::Pisa => vec![Location::Florence, Location::Rome, Location::Genoa],
			Location::Florence => vec![Location::Bologna, Location::SanMarino, Location::Perugia, Location::Pisa],
			Location::SanMarino => vec![Location::Bologna, Location::Florence],
			Location::Perugia => vec![Location::Florence, Location::Rome],
			Location::Rome => vec![Location::Perugia, Location::Pisa],
			Location::Gdansk => vec![Location::Szczecin, Location::Bydgoszcz],
			Location::Szczecin => vec![Location::Gdansk, Location::Bydgoszcz, Location::Poznan, Location::Berlin],
			Location::Bydgoszcz => vec![Location::Gdansk, Location::Poznan, Location::Szczecin],
			Location::Poznan => vec![Location::Szczecin, Location::Bydgoszcz, Location::Wroclaw, Location::Berlin],
			Location::Wroclaw => vec![Location::Poznan, Location::Ostrava, Location::Liberec],
			Location::Pilsen => vec![Location::Prague, Location::CeskeBudejovice, Location::Nuremberg],
			Location::Prague => vec![Location::Liberec, Location::Brno, Location::Pilsen, Location::Dresden],
			Location::Liberec => vec![Location::Wroclaw, Location::Prague],
			Location::CeskeBudejovice => vec![Location::Pilsen, Location::Linz],
			Location::Brno => vec![Location::Ostrava, Location::Vienna, Location::Prague],
			Location::Ostrava => vec![Location::Wroclaw, Location::Brno],
			Location::Sopron => vec![Location::Vienna, Location::Graz],
			Location::Ljubljana => vec![Location::Villach, Location::Zagreb, Location::Rijeka, Location::Venice],
			Location::Rijeka => vec![Location::Ljubljana, Location::Zagreb, Location::Split],
			Location::Zagreb => vec![Location::BanjaLuka, Location::Rijeka, Location::Ljubljana],
			Location::Split => vec![Location::BanjaLuka, Location::Rijeka],
			Location::BanjaLuka => vec![Location::Zagreb, Location::Split],
		}
	}

	pub fn get_high_speed_connections(&self) -> Vec<Location> {
		match self {
			Location::Dublin => vec![],
			Location::Rosslare => vec![],
			Location::Belfast => vec![],
			Location::Cairnryan => vec![],
			Location::Glasgow => vec![],
			Location::Edinburgh => vec![Location::York, Location::Liverpool],
			Location::Newcastle => vec![],
			Location::York => vec![Location::Edinburgh, Location::London],
			Location::Liverpool => vec![Location::Edinburgh, Location::London],
			Location::Holyhead => vec![],
			Location::Fishguard => vec![],
			Location::Swansea => vec![],
			Location::Birmingham => vec![],
			Location::Nottingham => vec![],
			Location::Cambridge => vec![],
			Location::Oxford => vec![],
			Location::Plymouth => vec![],
			Location::Bournemouth => vec![],
			Location::London => vec![Location::York, Location::Liverpool, Location::Paris],
			Location::Aalborg => vec![],
			Location::Aarhus => vec![],
			Location::Esbjerg => vec![],
			Location::Copenhagen => vec![Location::Hamburg],
			Location::Bilbao => vec![],
			Location::Burgos => vec![],
			Location::Pamplona => vec![],
			Location::Valladolid => vec![],
			Location::Zaragoza => vec![Location::Madrid, Location::Toulouse],
			Location::Madrid => vec![Location::Zaragoza, Location::Barcelona, Location::Toulouse],
			Location::Albacete => vec![],
			Location::Valencia => vec![],
			Location::Barcelona => vec![Location::Madrid, Location::Bordeaux],
			Location::Andorra => vec![],
			Location::Calais => vec![],
			Location::LeHavre => vec![],
			Location::Paris => vec![Location::London, Location::Brussels, Location::Frankfurt, Location::Lyon, Location::Toulouse, Location::Bordeaux, Location::Nantes],
			Location::CharlevilleMezieres => vec![],
			Location::Brest => vec![],
			Location::Rennes => vec![],
			Location::Nantes => vec![Location::Paris, Location::Bordeaux],
			Location::LeMans => vec![],
			Location::Orleans => vec![],
			Location::Poitiers => vec![],
			Location::LaRochelle => vec![],
			Location::Limoges => vec![],
			Location::Bordeaux => vec![Location::Nantes, Location::Barcelona, Location::Marseille, Location::Paris],
			Location::ClermontFerrand => vec![],
			Location::Toulouse => vec![Location::Paris, Location::Madrid, Location::Zaragoza],
			Location::Montpellier => vec![],
			Location::Nancy => vec![],
			Location::Strasbourg => vec![Location::Frankfurt],
			Location::Dijon => vec![],
			Location::Lyon => vec![Location::Paris, Location::Basel, Location::Marseille],
			Location::Grenoble => vec![],
			Location::Marseille => vec![Location::Bordeaux, Location::Lyon, Location::Milan],
			Location::Nice => vec![],
			Location::Groningen => vec![],
			Location::Amsterdam => vec![Location::Hamburg, Location::Brussels],
			Location::TheHague => vec![],
			Location::SHertogenbosch => vec![],
			Location::Ghent => vec![],
			Location::Antwerp => vec![],
			Location::Brussels => vec![Location::Amsterdam, Location::Frankfurt, Location::Paris],
			Location::Luxembourg => vec![],
			Location::Kiel => vec![],
			Location::Bremen => vec![],
			Location::Hamburg => vec![Location::Copenhagen, Location::Rostock, Location::Frankfurt, Location::Amsterdam],
			Location::Rostock => vec![Location::Berlin, Location::Hamburg],
			Location::Bielefeld => vec![],
			Location::Magdeburg => vec![],
			Location::Berlin => vec![Location::Rostock, Location::Poznan, Location::Dresden, Location::Frankfurt],
			Location::Cologne => vec![],
			Location::Kassel => vec![],
			Location::Erfurt => vec![],
			Location::Leipzig => vec![],
			Location::Dresden => vec![Location::Berlin, Location::Munich],
			Location::Frankfurt => vec![Location::Hamburg, Location::Berlin, Location::Munich, Location::Stuttgart, Location::Strasbourg, Location::Paris, Location::Brussels],
			Location::Nuremberg => vec![],
			Location::Stuttgart => vec![Location::Frankfurt, Location::Basel],
			Location::Munich => vec![Location::Dresden, Location::Prague, Location::Vienna, Location::Frankfurt],
			Location::Basel => vec![Location::Stuttgart, Location::Milan, Location::Lyon],
			Location::Zurich => vec![],
			Location::Merlischachen => vec![],
			Location::Geneva => vec![],
			Location::Innsbruck => vec![Location::Vienna, Location::Venice],
			Location::Salzburg => vec![],
			Location::Linz => vec![],
			Location::Vienna => vec![Location::Innsbruck, Location::Munich],
			Location::Villach => vec![],
			Location::Graz => vec![],
			Location::Bolzano => vec![],
			Location::Trento => vec![],
			Location::Turin => vec![],
			Location::Milan => vec![Location::Basel, Location::Venice, Location::Rome, Location::Marseille],
			Location::Padua => vec![],
			Location::Venice => vec![Location::Milan, Location::Rome, Location::Innsbruck],
			Location::Genoa => vec![],
			Location::Bologna => vec![],
			Location::Pisa => vec![],
			Location::Florence => vec![],
			Location::SanMarino => vec![],
			Location::Perugia => vec![],
			Location::Rome => vec![Location::Milan, Location::Venice],
			Location::Gdansk => vec![],
			Location::Szczecin => vec![],
			Location::Bydgoszcz => vec![],
			Location::Poznan => vec![Location::Berlin, Location::Prague],
			Location::Wroclaw => vec![],
			Location::Pilsen => vec![],
			Location::Prague => vec![Location::Poznan, Location::Munich],
			Location::Liberec => vec![],
			Location::CeskeBudejovice => vec![],
			Location::Brno => vec![],
			Location::Ostrava => vec![],
			Location::Sopron => vec![],
			Location::Ljubljana => vec![],
			Location::Rijeka => vec![],
			Location::Zagreb => vec![],
			Location::Split => vec![],
			Location::BanjaLuka => vec![],
		}
	}

	pub fn get_plane_connections(&self) -> Vec<Location> {
		match self {
				Location::Dublin => vec![Location::Copenhagen, Location::London, Location::Paris],
				Location::Rosslare => vec![],
				Location::Belfast => vec![],
				Location::Cairnryan => vec![],
				Location::Glasgow => vec![],
				Location::Edinburgh => vec![],
				Location::Newcastle => vec![],
				Location::York => vec![],
				Location::Liverpool => vec![],
				Location::Holyhead => vec![],
				Location::Fishguard => vec![],
				Location::Swansea => vec![],
				Location::Birmingham => vec![],
				Location::Nottingham => vec![],
				Location::Cambridge => vec![],
				Location::Oxford => vec![],
				Location::Plymouth => vec![],
				Location::Bournemouth => vec![],
				Location::London => vec![Location::Dublin, Location::Berlin, Location::Frankfurt, Location::Paris],
				Location::Aalborg => vec![],
				Location::Aarhus => vec![],
				Location::Esbjerg => vec![],
				Location::Copenhagen => vec![Location::Dublin, Location::Frankfurt, Location::Vienna],
				Location::Bilbao => vec![],
				Location::Burgos => vec![],
				Location::Pamplona => vec![],
				Location::Valladolid => vec![],
				Location::Zaragoza => vec![],
				Location::Madrid => vec![Location::Paris, Location::Rome],
				Location::Albacete => vec![],
				Location::Valencia => vec![],
				Location::Barcelona => vec![],
				Location::Andorra => vec![],
				Location::Calais => vec![],
				Location::LeHavre => vec![],
				Location::Paris => vec![Location::Dublin, Location::London, Location::Berlin, Location::Zurich, Location::Madrid],
				Location::CharlevilleMezieres => vec![],
				Location::Brest => vec![],
				Location::Rennes => vec![],
				Location::Nantes => vec![],
				Location::LeMans => vec![],
				Location::Orleans => vec![],
				Location::Poitiers => vec![],
				Location::LaRochelle => vec![],
				Location::Limoges => vec![],
				Location::Bordeaux => vec![],
				Location::ClermontFerrand => vec![],
				Location::Toulouse => vec![],
				Location::Montpellier => vec![],
				Location::Nancy => vec![],
				Location::Strasbourg => vec![],
				Location::Dijon => vec![],
				Location::Lyon => vec![],
				Location::Grenoble => vec![],
				Location::Marseille => vec![],
				Location::Nice => vec![],
				Location::Groningen => vec![],
				Location::Amsterdam => vec![],
				Location::TheHague => vec![],
				Location::SHertogenbosch => vec![],
				Location::Ghent => vec![],
				Location::Antwerp => vec![],
				Location::Brussels => vec![],
				Location::Luxembourg => vec![],
				Location::Kiel => vec![],
				Location::Bremen => vec![],
				Location::Hamburg => vec![],
				Location::Rostock => vec![],
				Location::Bielefeld => vec![],
				Location::Magdeburg => vec![],
				Location::Berlin => vec![Location::Vienna, Location::Paris, Location::London],
				Location::Cologne => vec![],
				Location::Kassel => vec![],
				Location::Erfurt => vec![],
				Location::Leipzig => vec![],
				Location::Dresden => vec![],
				Location::Frankfurt => vec![Location::Copenhagen, Location::Zurich, Location::London],
				Location::Nuremberg => vec![],
				Location::Stuttgart => vec![],
				Location::Munich => vec![],
				Location::Basel => vec![],
				Location::Zurich => vec![Location::Frankfurt, Location::Rome, Location::Paris],
				Location::Merlischachen => vec![],
				Location::Geneva => vec![],
				Location::Innsbruck => vec![],
				Location::Salzburg => vec![],
				Location::Linz => vec![],
				Location::Vienna => vec![Location::Berlin, Location::Copenhagen, Location::Rome],
				Location::Villach => vec![],
				Location::Graz => vec![],
				Location::Bolzano => vec![],
				Location::Trento => vec![],
				Location::Turin => vec![],
				Location::Milan => vec![],
				Location::Padua => vec![],
				Location::Venice => vec![],
				Location::Genoa => vec![],
				Location::Bologna => vec![],
				Location::Pisa => vec![],
				Location::Florence => vec![],
				Location::SanMarino => vec![],
				Location::Perugia => vec![],
				Location::Rome => vec![Location::Zurich, Location::Vienna, Location::Madrid],
				Location::Gdansk => vec![],
				Location::Szczecin => vec![],
				Location::Bydgoszcz => vec![],
				Location::Poznan => vec![],
				Location::Wroclaw => vec![],
				Location::Pilsen => vec![],
				Location::Prague => vec![],
				Location::Liberec => vec![],
				Location::CeskeBudejovice => vec![],
				Location::Brno => vec![],
				Location::Ostrava => vec![],
				Location::Sopron => vec![],
				Location::Ljubljana => vec![],
				Location::Rijeka => vec![],
				Location::Zagreb => vec![],
				Location::Split => vec![],
				Location::BanjaLuka => vec![],
		}
	}

	pub fn get_joker_connections(&self) -> Vec<Location> {
		return vec![self.get_low_speed_connections(), self.get_high_speed_connections(), self.get_plane_connections()].into_iter().flatten().collect();
	}

	pub fn is_coin_field(&self) -> bool {
		match self {
			Location::Edinburgh => true,
			Location::Swansea => true,
			Location::Plymouth => true,
			Location::Aalborg => true,
			Location::Bilbao => true,
			Location::Valencia => true,
			Location::LeHavre => true,
			Location::LaRochelle => true,
			Location::Montpellier => true,
			Location::Groningen => true,
			Location::SHertogenbosch => true,
			Location::Magdeburg => true,
			Location::Stuttgart => true,
			Location::Merlischachen => true,
			Location::Geneva => true,
			Location::Gdansk => true,
			Location::Prague => true,
			Location::Graz => true,
			Location::Rijeka => true,
			Location::Trento => true,
			Location::Pisa => true,
			_ => false,
		}
	}

	pub fn is_event_field(&self) -> bool {
		match self {
			Location::Holyhead => true,
			Location::Andorra => true,
			Location::Brest => true,
			Location::ClermontFerrand => true,
			Location::Ghent => true,
			Location::Bielefeld => true,
			Location::Leipzig => true,
			Location::Szczecin => true,
			Location::Ostrava => true,
			Location::Zagreb => true,
			Location::Bologna => true,
			_ => false,
		}
	}

	fn get_iter() -> impl Iterator<Item = Location> {
		return vec![
				Location::Dublin,
				Location::Rosslare,
				Location::Belfast,
				Location::Cairnryan,
				Location::Glasgow,
				Location::Edinburgh,
				Location::Newcastle,
				Location::York,
				Location::Liverpool,
				Location::Holyhead,
				Location::Fishguard,
				Location::Swansea,
				Location::Birmingham,
				Location::Nottingham,
				Location::Cambridge,
				Location::Oxford,
				Location::Plymouth,
				Location::Bournemouth,
				Location::London,
				Location::Aalborg,
				Location::Aarhus,
				Location::Esbjerg,
				Location::Copenhagen,
				Location::Bilbao,
				Location::Burgos,
				Location::Pamplona,
				Location::Valladolid,
				Location::Zaragoza,
				Location::Madrid,
				Location::Albacete,
				Location::Valencia,
				Location::Barcelona,
				Location::Andorra,
				Location::Calais,
				Location::LeHavre,
				Location::Paris,
				Location::CharlevilleMezieres,
				Location::Brest,
				Location::Rennes,
				Location::Nantes,
				Location::LeMans,
				Location::Orleans,
				Location::Poitiers,
				Location::LaRochelle,
				Location::Limoges,
				Location::Bordeaux,
				Location::ClermontFerrand,
				Location::Toulouse,
				Location::Montpellier,
				Location::Nancy,
				Location::Strasbourg,
				Location::Dijon,
				Location::Lyon,
				Location::Grenoble,
				Location::Marseille,
				Location::Nice,
				Location::Groningen,
				Location::Amsterdam,
				Location::TheHague,
				Location::SHertogenbosch,
				Location::Ghent,
				Location::Antwerp,
				Location::Brussels,
				Location::Luxembourg,
				Location::Kiel,
				Location::Bremen,
				Location::Hamburg,
				Location::Rostock,
				Location::Bielefeld,
				Location::Magdeburg,
				Location::Berlin,
				Location::Cologne,
				Location::Kassel,
				Location::Erfurt,
				Location::Leipzig,
				Location::Dresden,
				Location::Frankfurt,
				Location::Nuremberg,
				Location::Stuttgart,
				Location::Munich,
				Location::Basel,
				Location::Zurich,
				Location::Merlischachen,
				Location::Geneva,
				Location::Innsbruck,
				Location::Salzburg,
				Location::Linz,
				Location::Vienna,
				Location::Villach,
				Location::Graz,
				Location::Bolzano,
				Location::Trento,
				Location::Turin,
				Location::Milan,
				Location::Padua,
				Location::Venice,
				Location::Genoa,
				Location::Bologna,
				Location::Pisa,
				Location::Florence,
				Location::SanMarino,
				Location::Perugia,
				Location::Rome,
				Location::Gdansk,
				Location::Szczecin,
				Location::Bydgoszcz,
				Location::Poznan,
				Location::Wroclaw,
				Location::Pilsen,
				Location::Prague,
				Location::Liberec,
				Location::CeskeBudejovice,
				Location::Brno,
				Location::Ostrava,
				Location::Sopron,
				Location::Ljubljana,
				Location::Rijeka,
				Location::Zagreb,
				Location::Split,
				Location::BanjaLuka,
		].into_iter();
	}
}

impl Display for Location {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Location::Dublin => write!(f, "dublin"),
			Location::Rosslare => write!(f, "rosslare"),
			Location::Belfast => write!(f, "belfast"),
			Location::Cairnryan => write!(f, "cairnryan"),
			Location::Glasgow => write!(f, "glasgow"),
			Location::Edinburgh => write!(f, "edinburgh"),
			Location::Newcastle => write!(f, "newcastle"),
			Location::York => write!(f, "york"),
			Location::Liverpool => write!(f, "liverpool"),
			Location::Holyhead => write!(f, "holyhead"),
			Location::Fishguard => write!(f, "fishguard"),
			Location::Swansea => write!(f, "swansea"),
			Location::Birmingham => write!(f, "birmingham"),
			Location::Nottingham => write!(f, "nottingham"),
			Location::Cambridge => write!(f, "cambridge"),
			Location::Oxford => write!(f, "oxford"),
			Location::Plymouth => write!(f, "plymouth"),
			Location::Bournemouth => write!(f, "bournemouth"),
			Location::London => write!(f, "london"),
			Location::Aalborg => write!(f, "aalborg"),
			Location::Aarhus => write!(f, "aarhus"),
			Location::Esbjerg => write!(f, "esbjerg"),
			Location::Copenhagen => write!(f, "copenhagen"),
			Location::Bilbao => write!(f, "bilbao"),
			Location::Burgos => write!(f, "burgos"),
			Location::Pamplona => write!(f, "pamplona"),
			Location::Valladolid => write!(f, "valladolid"),
			Location::Zaragoza => write!(f, "zaragoza"),
			Location::Madrid => write!(f, "madrid"),
			Location::Albacete => write!(f, "albacete"),
			Location::Valencia => write!(f, "valencia"),
			Location::Barcelona => write!(f, "barcelona"),
			Location::Andorra => write!(f, "andorra"),
			Location::Calais => write!(f, "calais"),
			Location::LeHavre => write!(f, "le_havre"),
			Location::Paris => write!(f, "paris"),
			Location::CharlevilleMezieres => write!(f, "charleville_mezieres"),
			Location::Brest => write!(f, "brest"),
			Location::Rennes => write!(f, "rennes"),
			Location::Nantes => write!(f, "nantes"),
			Location::LeMans => write!(f, "le_mans"),
			Location::Orleans => write!(f, "orleans"),
			Location::Poitiers => write!(f, "poitiers"),
			Location::LaRochelle => write!(f, "la_rochelle"),
			Location::Limoges => write!(f, "limoges"),
			Location::Bordeaux => write!(f, "bordeaux"),
			Location::ClermontFerrand => write!(f, "clermont_ferrand"),
			Location::Toulouse => write!(f, "toulouse"),
			Location::Montpellier => write!(f, "montpellier"),
			Location::Nancy => write!(f, "nancy"),
			Location::Strasbourg => write!(f, "strasbourg"),
			Location::Dijon => write!(f, "dijon"),
			Location::Lyon => write!(f, "lyon"),
			Location::Grenoble => write!(f, "grenoble"),
			Location::Marseille => write!(f, "marseille"),
			Location::Nice => write!(f, "nice"),
			Location::Groningen => write!(f, "groningen"),
			Location::Amsterdam => write!(f, "amsterdam"),
			Location::TheHague => write!(f, "the_hague"),
			Location::SHertogenbosch => write!(f, "s_hertogenbosch"),
			Location::Ghent => write!(f, "ghent"),
			Location::Antwerp => write!(f, "antwerp"),
			Location::Brussels => write!(f, "brussels"),
			Location::Luxembourg => write!(f, "luxembourg"),
			Location::Kiel => write!(f, "kiel"),
			Location::Bremen => write!(f, "bremen"),
			Location::Hamburg => write!(f, "hamburg"),
			Location::Rostock => write!(f, "rostock"),
			Location::Bielefeld => write!(f, "bielefeld"),
			Location::Magdeburg => write!(f, "magdeburg"),
			Location::Berlin => write!(f, "berlin"),
			Location::Cologne => write!(f, "cologne"),
			Location::Kassel => write!(f, "kassel"),
			Location::Erfurt => write!(f, "erfurt"),
			Location::Leipzig => write!(f, "leipzig"),
			Location::Dresden => write!(f, "dresden"),
			Location::Frankfurt => write!(f, "frankfurt"),
			Location::Nuremberg => write!(f, "nuremberg"),
			Location::Stuttgart => write!(f, "stuttgart"),
			Location::Munich => write!(f, "munich"),
			Location::Basel => write!(f, "basel"),
			Location::Zurich => write!(f, "zurich"),
			Location::Merlischachen => write!(f, "merlischachen"),
			Location::Geneva => write!(f, "geneva"),
			Location::Innsbruck => write!(f, "innsbruck"),
			Location::Salzburg => write!(f, "salzburg"),
			Location::Linz => write!(f, "linz"),
			Location::Vienna => write!(f, "vienna"),
			Location::Villach => write!(f, "villach"),
			Location::Graz => write!(f, "graz"),
			Location::Bolzano => write!(f, "bolzano"),
			Location::Trento => write!(f, "trento"),
			Location::Turin => write!(f, "turin"),
			Location::Milan => write!(f, "milan"),
			Location::Padua => write!(f, "padua"),
			Location::Venice => write!(f, "venice"),
			Location::Genoa => write!(f, "genoa"),
			Location::Bologna => write!(f, "bologna"),
			Location::Pisa => write!(f, "pisa"),
			Location::Florence => write!(f, "florence"),
			Location::SanMarino => write!(f, "san_marino"),
			Location::Perugia => write!(f, "perugia"),
			Location::Rome => write!(f, "rome"),
			Location::Gdansk => write!(f, "gdansk"),
			Location::Szczecin => write!(f, "szczecin"),
			Location::Bydgoszcz => write!(f, "bydgoszcz"),
			Location::Poznan => write!(f, "poznan"),
			Location::Wroclaw => write!(f, "wroclaw"),
			Location::Pilsen => write!(f, "pilsen"),
			Location::Prague => write!(f, "prague"),
			Location::Liberec => write!(f, "liberec"),
			Location::CeskeBudejovice => write!(f, "ceske_budejovice"),
			Location::Brno => write!(f, "brno"),
			Location::Ostrava => write!(f, "ostrava"),
			Location::Sopron => write!(f, "sopron"),
			Location::Ljubljana => write!(f, "ljubljana"),
			Location::Rijeka => write!(f, "rijeka"),
			Location::Zagreb => write!(f, "zagreb"),
			Location::Split => write!(f, "split"),
			Location::BanjaLuka => write!(f, "banja_luka"),
		}
	}
}

impl From<String> for Location {
	fn from(value: String) -> Self {
		match value.as_str() {
			"dublin" => Location::Dublin,
			"rosslare" => Location::Rosslare,
			"belfast" => Location::Belfast,
			"cairnryan" => Location::Cairnryan,
			"glasgow" => Location::Glasgow,
			"edinburgh" => Location::Edinburgh,
			"newcastle" => Location::Newcastle,
			"york" => Location::York,
			"liverpool" => Location::Liverpool,
			"holyhead" => Location::Holyhead,
			"fishguard" => Location::Fishguard,
			"swansea" => Location::Swansea,
			"birmingham" => Location::Birmingham,
			"nottingham" => Location::Nottingham,
			"cambridge" => Location::Cambridge,
			"oxford" => Location::Oxford,
			"plymouth" => Location::Plymouth,
			"bournemouth" => Location::Bournemouth,
			"london" => Location::London,
			"aalborg" => Location::Aalborg,
			"aarhus" => Location::Aarhus,
			"esbjerg" => Location::Esbjerg,
			"copenhagen" => Location::Copenhagen,
			"bilbao" => Location::Bilbao,
			"burgos" => Location::Burgos,
			"pamplona" => Location::Pamplona,
			"valladolid" => Location::Valladolid,
			"zaragoza" => Location::Zaragoza,
			"madrid" => Location::Madrid,
			"albacete" => Location::Albacete,
			"valencia" => Location::Valencia,
			"barcelona" => Location::Barcelona,
			"andorra" => Location::Andorra,
			"calais" => Location::Calais,
			"le_havre" => Location::LeHavre,
			"paris" => Location::Paris,
			"charleville_mezieres" => Location::CharlevilleMezieres,
			"brest" => Location::Brest,
			"rennes" => Location::Rennes,
			"nantes" => Location::Nantes,
			"le_mans" => Location::LeMans,
			"orleans" => Location::Orleans,
			"poitiers" => Location::Poitiers,
			"la_rochelle" => Location::LaRochelle,
			"limoges" => Location::Limoges,
			"bordeaux" => Location::Bordeaux,
			"clermont_ferrand" => Location::ClermontFerrand,
			"toulouse" => Location::Toulouse,
			"montpellier" => Location::Montpellier,
			"nancy" => Location::Nancy,
			"strasbourg" => Location::Strasbourg,
			"dijon" => Location::Dijon,
			"lyon" => Location::Lyon,
			"grenoble" => Location::Grenoble,
			"marseille" => Location::Marseille,
			"nice" => Location::Nice,
			"groningen" => Location::Groningen,
			"amsterdam" => Location::Amsterdam,
			"the_hague" => Location::TheHague,
			"s_hertogenbosch" => Location::SHertogenbosch,
			"ghent" => Location::Ghent,
			"antwerp" => Location::Antwerp,
			"brussels" => Location::Brussels,
			"luxembourg" => Location::Luxembourg,
			"kiel" => Location::Kiel,
			"bremen" => Location::Bremen,
			"hamburg" => Location::Hamburg,
			"rostock" => Location::Rostock,
			"bielefeld" => Location::Bielefeld,
			"magdeburg" => Location::Magdeburg,
			"berlin" => Location::Berlin,
			"cologne" => Location::Cologne,
			"kassel" => Location::Kassel,
			"erfurt" => Location::Erfurt,
			"leipzig" => Location::Leipzig,
			"dresden" => Location::Dresden,
			"frankfurt" => Location::Frankfurt,
			"nuremberg" => Location::Nuremberg,
			"stuttgart" => Location::Stuttgart,
			"munich" => Location::Munich,
			"basel" => Location::Basel,
			"zurich" => Location::Zurich,
			"merlischachen" => Location::Merlischachen,
			"geneva" => Location::Geneva,
			"innsbruck" => Location::Innsbruck,
			"salzburg" => Location::Salzburg,
			"linz" => Location::Linz,
			"vienna" => Location::Vienna,
			"villach" => Location::Villach,
			"graz" => Location::Graz,
			"bolzano" => Location::Bolzano,
			"trento" => Location::Trento,
			"turin" => Location::Turin,
			"milan" => Location::Milan,
			"padua" => Location::Padua,
			"venice" => Location::Venice,
			"genoa" => Location::Genoa,
			"bologna" => Location::Bologna,
			"pisa" => Location::Pisa,
			"florence" => Location::Florence,
			"san_marino" => Location::SanMarino,
			"perugia" => Location::Perugia,
			"rome" => Location::Rome,
			"gdansk" => Location::Gdansk,
			"szczecin" => Location::Szczecin,
			"bydgoszcz" => Location::Bydgoszcz,
			"poznan" => Location::Poznan,
			"wroclaw" => Location::Wroclaw,
			"pilsen" => Location::Pilsen,
			"prague" => Location::Prague,
			"liberec" => Location::Liberec,
			"ceske_budejovice" => Location::CeskeBudejovice,
			"brno" => Location::Brno,
			"ostrava" => Location::Ostrava,
			"sopron" => Location::Sopron,
			"ljubljana" => Location::Ljubljana,
			"rijeka" => Location::Rijeka,
			"zagreb" => Location::Zagreb,
			"split" => Location::Split,
			"banja_luka" => Location::BanjaLuka,
			_ => panic!("{value} not a valid Location ID"),
		}
	}
}

impl From<&str> for Location {
	fn from(value: &str) -> Self {
		return value.to_string().into();
	}
}