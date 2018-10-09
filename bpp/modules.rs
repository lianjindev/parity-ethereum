use std::sync::Arc;

use ethcore::client::BlockChainClient;
use sync::{self, AttachedProtocol, SyncConfig, NetworkConfiguration, Params, ConnectionFilter};
use ethcore::snapshot::SnapshotService;
use light::Provider;

pub use sync::{EthSync, SyncProvider, ManageNetwork, PrivateTxHandler};
pub use ethcore::client::ChainNotify;
use ethcore_logger::Config as LogConfig;

pub type SyncModules = (Arc<SyncProvider>, Arc<ManageNetwork>, Arc<ChainNotify>);

pub fn sync(
	sync_cfg: SyncConfig,
	net_cfg: NetworkConfiguration,
	client: Arc<BlockChainClient>,
	snapshot_service: Arc<SnapshotService>,
	private_tx_handler: Arc<PrivateTxHandler>,
	provider: Arc<Provider>,
	_log_settings: &LogConfig,
	attached_protos: Vec<AttachedProtocol>,
	connection_filter: Option<Arc<ConnectionFilter>>,
) -> Result<SyncModules, sync::Error> {
	let eth_sync = EthSync::new(Params {
		config: sync_cfg,
		chain: client,
		provider: provider,
		snapshot_service: snapshot_service,
		private_tx_handler,
		network_config: net_cfg,
		attached_protos: attached_protos,
	},
	connection_filter)?;

	Ok((eth_sync.clone() as Arc<SyncProvider>, eth_sync.clone() as Arc<ManageNetwork>, eth_sync.clone() as Arc<ChainNotify>))
}
