extern crate hyper;
extern crate hyper_router;

use hyper::{Body, Method, Request, Response, StatusCode};
use std::collections::HashMap;
use url::form_urlencoded;
use serde_json::{Value};

mod db;

static INDEX: &[u8] = b"<html><body><form action=\"post\" method=\"post\">Name: <input type=\"text\" name=\"name\"><br>Number: <input type=\"text\" name=\"number\"><br><input type=\"submit\"></body></html>";
static MISSING: &[u8] = b"Missing field";
static NOTNUMERIC: &[u8] = b"Number field is not numeric";

pub async fn param_example(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
	match (req.method(), req.uri().path()) {
			(&Method::GET, "/") | (&Method::GET, "/post") => Ok(Response::new(INDEX.into())),
			(&Method::POST, "/post") => {
					let whole_body = hyper::body::to_bytes(req.into_body()).await?;
					// println!("{:?}", whole_body.clone());
    			let body:Value = serde_json::from_slice(&(whole_body.clone())).unwrap();
					println!("{:?}", body);
					Ok(Response::new(whole_body.into()))
			}
			(&Method::GET, "/get") => {
					let query = if let Some(q) = req.uri().query() {
							q
					} else {
							return Ok(Response::builder()
									.status(StatusCode::UNPROCESSABLE_ENTITY)
									.body(MISSING.into())
									.unwrap());
					};
					let params = form_urlencoded::parse(query.as_bytes())
							.into_owned()
							.collect::<HashMap<String, String>>();
					let page = if let Some(p) = params.get("page") {
							p
					} else {
							return Ok(Response::builder()
									.status(StatusCode::UNPROCESSABLE_ENTITY)
									.body(MISSING.into())
									.unwrap());
					};
					let body = format!("You requested {}", page);
					Ok(Response::new(body.into()))
			}
			_ => Ok(Response::builder()
					.status(StatusCode::NOT_FOUND)
					.body(Body::empty())
					.unwrap()),
	}
}