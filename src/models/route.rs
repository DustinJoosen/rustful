use crate::models::request::Request;
use crate::models::response::Response;

#[derive(Debug, Clone)]
pub struct Route {
	pub uri: String,
	pub method: HttpMethod,
	pub action: fn(Request) -> Response
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpMethod {
	GET,
	POST,
	PUT,
	DELETE,
}
