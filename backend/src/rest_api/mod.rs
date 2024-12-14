use actix_web::{dev::Service as _, web, App, HttpServer, middleware};
use futures_util::future::FutureExt;

pub struct AppState {
	pub test: String,
}

pub async fn initialize_webserver() -> std::io::Result<()> {
	let api_port = 4000;
	println!("Starting webserver on port {}", api_port);
	return HttpServer::new(move|| {
		return App::new()
			.app_data(web::Data::new(AppState {
				test: "test".to_string(),
			}))
			.wrap(middleware::Compress::default())
			.wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
			.wrap_fn(|req, srv| {
				println!("req: {} {}", req.method(), req.path());
				srv.call(req).map(|res| {
					return res;
				})
			})
			.service(crate::game::rest_api::create_game);
	})
		.bind(("0.0.0.0", api_port))?
		.run()
		.await;
}