use std::net::TcpListener;
use colored::Colorize;
use crate::dispatcher::Dispatcher;
use crate::models::route::Route;


pub struct Server {
	address: String,
	routes: fn() -> Vec<Route>
}

impl Server {
	pub fn new(address: &str, routes: fn() -> Vec<Route>) -> Server {
		Server {
			address: address.to_string(),
			routes
		}
	}

	pub fn run(&self) {
		println!("Server started on {}", &self.address.blue());

		// Initiate listener.
		let listener = match TcpListener::bind(&self.address) {
			Ok(t) => t,
			Err(_) => panic!("Could not bind TcpListener with address {}", &self.address)
		};

		// Start listening for streams.
		for stream in listener.incoming() {
			let stream = match stream {
				Ok(t) => t,
				Err(_) => panic!("Could not find stream")
			};

			Dispatcher::dispatch(stream, self.routes)
		}
	}
}