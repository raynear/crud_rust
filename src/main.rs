use std::net::SocketAddr;
use hyper::{Server};
use hyper::service::{make_service_fn, service_fn};

use dotenv::dotenv;
use log::{error, info};

mod router;

#[tokio::main]
async fn main() {
	dotenv().ok();
	pretty_env_logger::init();

	let binding = std::env::var("IP").expect("IP must set on .env");
  let ip:[u8;4] = binding.split('.').map(|x| x.parse::<u8>().expect("IP should convert to u8")).collect::<Vec<u8>>().as_slice().try_into().expect("IP should convert [u8;4]");
	let port:u16 = std::env::var("PORT").expect("PORT must set on .env").parse().expect("PORT must set as uint");

	let addr = SocketAddr::from((ip, port));

	let server = Server::bind(&addr).serve(make_service_fn(|_| async {
		Ok::<_, hyper::Error>(service_fn(router::param_example))
		}));
	info!("server started on {:?}:{}", ip, port);

	if let Err(e) = server.await {
		error!("server error: {}", e);
	}
}


#[cfg(test)] // run module when testing, avoiding include library default
mod test {
	use super::*;

	#[test] // run by test runner
	fn snuggling_bunnies_multiply() {
		assert_eq!(2, 2);
	}
}
