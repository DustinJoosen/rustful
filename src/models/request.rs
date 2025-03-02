use std::collections::HashMap;
use crate::models::route::{HttpMethod, Route};
use crate::route_rover::RouteRover;

#[derive(Debug, Clone)]
pub struct Request {
	pub route: Option<Route>,
	pub uri: String,
	pub method: HttpMethod,
	pub headers: HashMap<String, String>,
	pub body: Option<String>,
}

impl Request {
	pub fn new(route: Option<Route>, uri: String, method: HttpMethod,
			   headers: HashMap<String, String>, body: Option<String>) -> Request {
		Request { route, uri, method, headers, body }
	}

	// Creates an empty request object.
	pub fn empty() -> Request {
		Request::new(None, String::from(""), HttpMethod::GET, HashMap::new(), None)
	}

	pub fn get_rover(&mut self) -> RouteRover {
		RouteRover::new(&self.uri)
	}

	pub fn rover_get(&mut self, key: &str) -> Option<String> {
		let rover = self.get_rover();
		let route = self.route.as_ref()?;

		rover.get(key, &route.uri)
	}

	pub fn rover_get_query(&mut self, key: &str) -> Option<String> {
		let rover = self.get_rover();
		rover.get_query(key)
	}
}
