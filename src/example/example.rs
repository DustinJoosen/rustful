
use syter6_rustful::{HttpMethod, Request, Response, Route, Server};

pub struct HomeController;

impl HomeController {
	pub fn find_user(mut request: Request) -> Response {
		let user_id = request.rover_get("user_id")
			.unwrap_or("User not found".to_string());

		// Do something with a database

		Response::ok(format!("{:?}", user_id).as_str())
	}
}

pub fn get_routes() -> Vec<Route> {
	vec![
		Route {
			uri: String::from("/user/{user_id}"),
			method: HttpMethod::GET,
			action: HomeController::find_user
		},
	]
}

fn main() {
	let server = Server::new("127.0.0.1:7878", get_routes);
	server.run();
}
