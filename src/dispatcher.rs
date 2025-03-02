use std::io::{Write};
use std::net::TcpStream;
use std::time::Instant;
use crate::models::route::{Route};
use crate::models::request::Request;
use crate::models::response::Response;
use crate::request_parser::RequestParser;
use crate::route_rover::RouteRover;

pub struct Dispatcher { }

impl Dispatcher {

	pub fn dispatch(mut stream: TcpStream, routes: fn() -> Vec<Route>) {
		let mut rp = RequestParser::new(stream.try_clone().unwrap());
		rp.parse_request();
		let mut request = rp.request;

		println!("{:?} {}", request.method, request.uri);

		let response_line = if let Some(matching_route) = Dispatcher::find_matching_action_route(&request, routes())  {
			// Make a copy of request for logging purposes.
			let request_clone = request.clone();

			// Start timer.
			let start_time = Instant::now();

			// Execute action and give it a request for metadata.
			request.route = Some(matching_route);
			let action_response = (request.route.as_ref().unwrap().action)(request);

			// Log result and format response.
			let time_in_ms = (Instant::now() - start_time).as_millis();
			println!("{} for {} in {}ms", action_response.get_status_line(), request_clone.uri, time_in_ms);

			Dispatcher::format_response(Some(action_response))
		} else {
			println!("404 Not Found for {} in 0ms", request.uri);
			Dispatcher::format_response(Some(Response::not_found(None)))
		};

		stream.write_all(response_line.as_bytes()).unwrap();
		stream.flush().unwrap();
	}

	fn format_response(response: Option<Response>) -> String {
		let response = response.unwrap_or(Response::not_found(None));

		let response_jsonified = serde_json::to_string(&response).unwrap();
		let response_body_length = response_jsonified.len();
		let response_status_line = format!("HTTP/1.1 {}", response.get_status_line());

		format!(
			"{response_status_line}\r\n\
    		Content-Type: application/json\r\n\
			Content-Length: {response_body_length}\r\n\
    		\r\n\
    		{response_jsonified}"
		)
	}

	fn find_matching_action_route(request: &Request, routes: Vec<Route>) -> Option<Route> {
		let rover = RouteRover::new(request.uri.as_str());

		for route in routes {
			if route.method != request.method {
				continue;
			}

			if rover.match_route(&route.uri) {
				return Some(route);
			}
		}

		None
	}
}