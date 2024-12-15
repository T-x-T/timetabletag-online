use actix_web::{dev::Service as _, web, App, HttpServer, middleware};
use actix_web::{get, HttpResponse, HttpRequest, Responder};
use futures_util::future::FutureExt;
use std::{collections::BTreeMap, sync::{Arc, Mutex}};
use uuid::Uuid;

use crate::game::Game;

pub struct AppState {
	pub games: Arc<Mutex<BTreeMap<Uuid, Game>>>,
	pub test: Arc<Mutex<usize>>,
}

pub async fn initialize_webserver() -> std::io::Result<()> {
	let api_port = 4000;
	println!("Starting webserver on port {}", api_port);

	let state = web::Data::new(AppState {
		games: Arc::new(Mutex::new(BTreeMap::new())),
		test: Arc::new(Mutex::new(0)),
	});

	return HttpServer::new(move|| {
		return App::new()
			.app_data(state.clone())
			.wrap(middleware::Compress::default())
			.wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
			.wrap_fn(|req, srv| {
				println!("req: {} {}", req.method(), req.path());
				srv.call(req).map(|res| {
					return res;
				})
			})
			.service(test)
			.service(crate::game::rest_api::join_game)
			.service(crate::game::rest_api::get_current_state)
			.service(crate::game::rest_api::create_game);
	})
		.bind(("0.0.0.0", api_port))?
		.run()
		.await;
}


#[get("/api/v1/test")]
pub async fn test(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
	let mut num = data.test.lock().unwrap();
	*num += 1;

	return HttpResponse::Ok().body(format!("{{\"test\":\"{num}\"}}"));
}