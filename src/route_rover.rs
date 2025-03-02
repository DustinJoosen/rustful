
#[derive(Debug, Clone)]
pub struct RouteRover {
	uri: String,
}

impl RouteRover {

	pub fn new(uri: &str) -> RouteRover {
		RouteRover { uri: uri.to_string() }
	}

	pub fn get(&self, key: &str, route_uri: &str) -> Option<String> {
		// Segment the route's and own uri. Return false if something went wrong.
		let Some((route_segments, uri_segments)) =
			&self.segment_route_and_own_uri(&route_uri) else {
			return None;
		};

		// Loop through each segment of both uri's.
		for (route_segment, uri_segment) in route_segments.iter().zip(uri_segments.iter()) {
			// If the route is enveloped in a '{}' then check the key.
			if route_segment.starts_with("{") && route_segment.ends_with("}") {

				let route_segment_param = &route_segment[1..route_segment.len() - 1];
				if route_segment_param == key {
					return Some(uri_segment.to_string());
				}
			}
		}

		None
	}

	pub fn get_query(&self, key: &str) -> Option<String> {
		// Get all queries (after the '?').
		let queries = &self.uri.split_once("?")?.1;
		if queries.is_empty() {
			return None
		}

		// Split the queries into key-value tuples.
		let query_list: Vec<(&str, &str)> = queries.split("&")
			.filter_map(|kv| kv.split_once("="))
			.collect();

		// If any of the tuples their keys match the request, send the value back.
		for (k, v) in query_list {
			if k == key { return Some(v.to_string()); }
		}

		None
	}


	pub fn match_route(&self, route: &str) -> bool {
		// If they are the same, obviously they match.
		if &self.uri == &route {
			return true;
		}

		// Segment the route's and own uri. Return false if something went wrong.
		let Some((route_segments, uri_segments)) =
			&self.segment_route_and_own_uri(&route) else {
			return false;
		};

		// Loop through each segment of both uri's.
		for (route_segment, uri_segment) in route_segments.iter().zip(uri_segments.iter()) {
			// If the segments are exactly the same, continue to the next one.
			if route_segment == uri_segment {
				continue;
			}

			// If the route is enveloped in a '{}' then it's still valid.
			if route_segment.starts_with("{") && route_segment.ends_with("}") {
				continue;
			}

			// The segments are not matching, so the route doesn't either.
			return false;
		}

		true
	}

	fn segment_route_and_own_uri(&self, route_uri: &str) -> Option<(Vec<String>, Vec<String>)> {
		// Clean the uri's. Remove trailing backslashes, and remove query parameters.
		let cleaned_route = &route_uri.trim_end_matches("/");
		let cleaned_uri = &self.uri
			.trim_end_matches("/")
			.split('?')
			.next()
			.unwrap_or("");

		// Prevent empty segments (e.g. /post//edit should not match with /post/{id}/edit)
		if cleaned_uri.contains("//") {
			return None;
		}

		// Get all segments of both uri's. Split based on '/'.
		let route_segments: Vec<String> = cleaned_route.split("/")
			.map(|s| s.to_string())
			.collect();
		let uri_segments: Vec<String> = cleaned_uri.split("/")
			.map(|s| s.to_string())
			.collect();

		// The routes don't have the same amount of segments. No way they match.
		if route_segments.len() != uri_segments.len() {
			return None;
		}

		Some((route_segments, uri_segments))
	}

}