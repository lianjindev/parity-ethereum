use ethcore::ethstore::{PresaleWallet, EthStore};
use ethcore::ethstore::accounts_dir::RootDiskDirectory;
use ethcore::account_provider::{AccountProvider, AccountProviderSettings};
use helpers::{password_prompt, password_from_file};
use params::SpecType;

#[derive(Debug, PartialEq)]
pub struct ImportWallet {
	pub iterations: u32,
	pub path: String,
	pub spec: SpecType,
	pub wallet_path: String,
	pub password_file: Option<String>,
}

pub fn execute(cmd: ImportWallet) -> Result<String, String> {
	let password = match cmd.password_file {
		Some(file) => password_from_file(file)?,
		None => password_prompt()?,
	};

	let dir = Box::new(RootDiskDirectory::create(cmd.path).unwrap());
	let secret_store = Box::new(EthStore::open_with_iterations(dir, cmd.iterations).unwrap());
	let acc_provider = AccountProvider::new(secret_store, AccountProviderSettings::default());
	let wallet = PresaleWallet::open(cmd.wallet_path).map_err(|_| "Unable to open presale wallet.")?;
	let kp = wallet.decrypt(&password).map_err(|_| "Invalid password.")?;
	let address = acc_provider.insert_account(kp.secret().clone(), &password).unwrap();
	Ok(format!("{:?}", address))
}
