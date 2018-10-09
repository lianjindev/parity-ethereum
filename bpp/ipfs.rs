use std::sync::Arc;
use parity_ipfs_api::{self, AccessControlAllowOrigin, Host, Listening};
use parity_ipfs_api::error::ServerError;
use ethcore::client::BlockChainClient;

#[derive(Debug, PartialEq, Clone)]
pub struct Configuration {
	pub enabled: bool,
	pub port: u16,
	pub interface: String,
	pub cors: Option<Vec<String>>,
	pub hosts: Option<Vec<String>>,
}

impl Default for Configuration {
	fn default() -> Self {
		Configuration {
			enabled: false,
			port: 5001,
			interface: "127.0.0.1".into(),
			cors: Some(vec![]),
			hosts: Some(vec![]),
		}
	}
}

pub fn start_server(conf: Configuration, client: Arc<BlockChainClient>) -> Result<Option<Listening>, ServerError> {
	if !conf.enabled {
		return Ok(None);
	}

	let cors = conf.cors.map(|cors| cors.into_iter().map(AccessControlAllowOrigin::from).collect());
	let hosts = conf.hosts.map(|hosts| hosts.into_iter().map(Host::from).collect());

	parity_ipfs_api::start_server(
		conf.port,
		conf.interface,
		cors.into(),
		hosts.into(),
		client
	).map(Some)
}
