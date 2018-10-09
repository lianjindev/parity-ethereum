use std::sync::{Arc, Weak};

use ethcore::encoded;
use ethcore::engines::{EthEngine, StateDependentProof};
use ethcore::header::Header;
use ethcore::machine::EthereumMachine;
use ethcore::receipt::Receipt;
use sync::LightSync;

use futures::{future, Future};
use futures::future::Either;

use light::client::fetch::ChainDataFetcher;
use light::on_demand::{request, OnDemand};

use parking_lot::RwLock;
use ethereum_types::H256;

const ALL_VALID_BACKREFS: &str = "no back-references, therefore all back-references valid; qed";

type BoxFuture<T, E> = Box<Future<Item = T, Error = E>>;

/// Allows on-demand fetch of data useful for the light client.
pub struct EpochFetch {
	/// A handle to the sync service.
	pub sync: Arc<RwLock<Weak<LightSync>>>,
	/// The on-demand request service.
	pub on_demand: Arc<OnDemand>,
}

impl EpochFetch {
	fn request<T>(&self, req: T) -> BoxFuture<T::Out, &'static str>
		where T: Send + request::RequestAdapter + 'static, T::Out: Send + 'static
	{
		Box::new(match self.sync.read().upgrade() {
			Some(sync) => {
				let on_demand = &self.on_demand;
				let maybe_future = sync.with_context(move |ctx| {
					on_demand.request(ctx, req).expect(ALL_VALID_BACKREFS)
				});

				match maybe_future {
					Some(x) => Either::A(x.map_err(|_| "Request canceled")),
					None => Either::B(future::err("Unable to access network.")),
				}
			}
			None => Either::B(future::err("Unable to access network")),
		})
	}
}

impl ChainDataFetcher for EpochFetch {
	type Error = &'static str;

	type Body = BoxFuture<encoded::Block, &'static str>;
	type Receipts = BoxFuture<Vec<Receipt>, &'static str>;
	type Transition = BoxFuture<Vec<u8>, &'static str>;

	fn block_body(&self, header: &Header) -> Self::Body {
		self.request(request::Body(header.encoded().into()))
	}

	/// Fetch block receipts.
	fn block_receipts(&self, header: &Header) -> Self::Receipts {
		self.request(request::BlockReceipts(header.encoded().into()))
	}

	/// Fetch epoch transition proof at given header.
	fn epoch_transition(&self, hash: H256, engine: Arc<EthEngine>, checker: Arc<StateDependentProof<EthereumMachine>>)
		-> Self::Transition
	{
		self.request(request::Signal {
			hash: hash,
			engine: engine,
			proof_check: checker,
		})
	}
}