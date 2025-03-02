# Syter6-Rustful

Syter6-Rustful is a lightweight, minimalistic api framework for Rust, designed to provide basic request handling, routing, and response management. It offers just enough functionality to build simple web applications without unnecessary complexity.

## Features
- Define and register routes easily
- Supports `GET`, `POST`, `PUT`, and `DELETE` methods
- Pass request metadata to controllers
- Return structured responses with status codes
- Minimal setup required

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
syter6-rustful = "0.1.2"
```

## Usage

#1 Create a controller with actions that will use
```rust

pub struct HomeController;

impl HomeController {
	pub fn find_user(request: Request) -> Response {
		let user_id = request.rover_get("user_id")
			.unwrap_or("User not found".to_string());
        
        // Do something with a database

		Response::ok(format!("Hello {:?}", user_id).as_str())
	}

	pub fn tea(_: Request) -> Response {
		Response::i_am_a_teapot()
	}
}

```

#2 Create a method to return a list of all routes. To add a parameter that can be found using
the route rover, add the name between brackets ('{' and '}')

```rust
pub fn get_routes() -> Vec<Route> {
	vec![
		Route {
			uri: String::from("/user/{user_id}"),
			method: HttpMethod::GET,
			action: HomeController::find_user
		},
		Route {
			uri: String::from("/teapot"),
			method: HttpMethod::GET,
			action: HomeController::tea
		}
	]
}
```

#3 Start the server. Pass along the routing method, and the address where the api can run on.
```rust
fn main() {
	let server = Server::new("127.0.0.1:7878", get_routes);
	server.run();
}
```
