use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
	pub status_code: u16,
	pub content: String
}

impl Response {
	pub fn new(status_code: u16, content: &str) -> Response {
		let content = content.to_string();
		Response { status_code, content }
	}

	pub fn get_status_line(&self) -> String {
		match self.status_code {
			200 => "200 OK".to_string(),
			201 => "201 Created".to_string(),
			400 => "400 Bad Request".to_string(),
			401 => "401 Unauthorized".to_string(),
			403 => "403 Forbidden".to_string(),
			404 => "404 Not Found".to_string(),
			405 => "405 Method Not Allowed".to_string(),
			418 => "418 I'm a Teapot".to_string(),
			500 => "500 Internal Server Error".to_string(),
			_ => "500 Internal Server Error".to_string(),
		}
	}
}


// Utility methods for the controllers QOL.
impl Response {
	pub fn ok(content: &str) -> Response {
		Self::new(200, content)
	}

	pub fn created(content: &str) -> Response {
		Self::new(201, content)
	}

	pub fn bad_request(content: Option<&str>) -> Response {
		Self::new(400, content.unwrap_or("Bad Request"))
	}

	pub fn unauthorized(content: Option<&str>) -> Response {
		Self::new(401, content.unwrap_or("Unauthorized"))
	}

	pub fn forbidden(content: Option<&str>) -> Response {
		Self::new(403, content.unwrap_or("Forbidden"))
	}

	pub fn not_found(content: Option<&str>) -> Response {
		Self::new(404, content.unwrap_or("Not Found"))
	}

	pub fn method_not_allowed(content: Option<&str>) -> Response {
		Self::new(405, content.unwrap_or("Method Not Allowed"))
	}

	pub fn i_am_a_teapot() -> Response {
		Self::new(418, "I'm a teapot")
	}

	pub fn internal_server_error(content: Option<&str>) -> Response {
		Self::new(500, content.unwrap_or("Internal Server Error"))
	}
}
