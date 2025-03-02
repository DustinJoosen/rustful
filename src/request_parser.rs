use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use crate::models::request::Request;
use crate::models::route::HttpMethod;


pub struct RequestParser {
	buf_reader: BufReader<TcpStream>,
	pub request: Request,
}

impl RequestParser {
	pub fn new(stream: TcpStream) -> RequestParser {
		RequestParser { buf_reader: BufReader::new(stream), request: Request::empty() }
	}

	pub fn parse_request(&mut self) -> Option<Request> {
		// Get the method and uri.
		let (method, uri) = self.get_method_and_uri()?;
		self.request.method = method;
		self.request.uri = uri;

		// Get the headers.
		let headers = self.get_headers();
		self.request.headers = headers;

		// Get the body (if applicable).
		if self.request.method == HttpMethod::POST || self.request.method == HttpMethod::PUT {
			self.request.body = self.get_body();
		}

		println!("{:?}", self.request);

		None
	}


	// Get the body content out of the rest of the lines.
	fn get_body(&mut self) -> Option<String> {
		// Find the content length.
		let Some(content_length) = self.request.headers.get("Content-Length") else {
			println!("No Content-Length header found!");
			return None;
		};

		// Parse the content length.
		let Ok(content_length) = content_length.parse::<usize>() else {
			println!("Invalid Content-Length header!");
			return None;
		};

		let mut body = vec![0; content_length];
		if let Err(e) = self.buf_reader.read_exact(&mut body) {
			println!("Failed to read body: {}", e);
			return None;
		}

		Some(self.parse_body(body))
	}

	fn parse_body(&self, body: Vec<u8>) -> String {
		String::from_utf8(body).expect("Body translation had issues")
	}

	// Gets all headers of the request. It goes until it reaches an empty line.
	fn get_headers(&mut self) -> HashMap<String, String> {
		let lines = self.buf_reader.by_ref().lines();
		let mut headers: HashMap<String, String> = HashMap::new();

		for line in lines.map_while(Result::ok) {
			if line.is_empty() {
				break;
			}

			if let Some((key, value)) = line.split_once(": ") {
				headers.insert(key.to_string(), value.to_string());
			}
		}

		headers

	}

	// This method deals mostly with unpacking the first line. Then it calls parse_first_req_line
	// -to parse the actual information.
	fn get_method_and_uri(&mut self)
		-> Option<(HttpMethod, String)> {
		let mut lines = self.buf_reader.by_ref().lines();

		// Try to extract the request line out of the reader.
		let first_line = match lines.next() {
			Some(v) => match v {
				Ok(v) => v,
				Err(_) => {
					println!("Could not unpack value out of result");
					return None;
				}
			},
			None => {
				println!("Could not unpack value out of option");
				return None;
			}
		};

		// Parse the request line.
		let (method, uri) = match self.parse_first_req_line(&first_line) {
			Some(v) => v,
			None => {
				println!("Could not parse request line");
				return None;
			}
		};

		Some((method, uri.to_string()))
	}

	fn parse_first_req_line(&self, request_line: &str) -> Option<(HttpMethod, String)> {
		let mut segments = request_line.split_whitespace();

		// Grab the method out of the first segment.
		let method = match segments.next()? {
			"GET" => HttpMethod::GET,
			"POST" => HttpMethod::POST,
			"PUT" => HttpMethod::PUT,
			"DELETE" => HttpMethod::DELETE,
			_ => return None
		};

		// Grab the uri out of the second segment.
		let uri = segments.next()?;

		Some((method, uri.to_string()))
	}
}


