use boltz_client::util::error::S5Error;
use lwk_signer::SwSigner;
use lwk_wollet::{ElectrumUrl, ElementsNetwork};

pub enum Network {
    Liquid,
    LiquidTestnet,
}

impl From<Network> for ElementsNetwork {
    fn from(value: Network) -> Self {
        match value {
            Network::Liquid => ElementsNetwork::Liquid,
            Network::LiquidTestnet => ElementsNetwork::LiquidTestnet,
        }
    }
}

pub struct WalletOptions {
    pub signer: SwSigner,
    pub network: Network,
    pub desc: String,
    pub db_root_path: Option<String>,
    pub chain_cache_path: Option<String>,
    pub electrum_url: Option<ElectrumUrl>,
}

#[derive(Debug)]
pub struct SwapLbtcResponse {
    pub id: String,
    pub invoice: String,
}

pub enum SwapStatus {
    Created,
    Mempool,
    Completed,
}

impl ToString for SwapStatus {
    fn to_string(&self) -> String {
        match self {
            SwapStatus::Mempool => "transaction.mempool",
            SwapStatus::Completed => "transaction.mempool",
            SwapStatus::Created => "swap.created",
        }
        .to_string()
    }
}

pub struct SendPaymentResponse {
    pub txid: String,
}

#[derive(thiserror::Error, Debug)]
pub enum SwapError {
    #[error("Could not contact Boltz servers: {err}")]
    ServersUnreachable { err: String },

    #[error("Invoice amount is out of range")]
    AmountOutOfRange,

    #[error("Wrong response received from Boltz servers")]
    BadResponse,

    #[error("The specified invoice is not valid")]
    InvalidInvoice,

    #[error("Could not sign/send the transaction")]
    SendError,

    #[error("Could not fetch the required wallet information")]
    WalletError,

    #[error("Could not store the swap details locally")]
    PersistError,

    #[error("Generic boltz error: {err}")]
    BoltzGeneric { err: String },
}

impl From<S5Error> for SwapError {
    fn from(err: S5Error) -> Self {
        match err.kind {
            boltz_client::util::error::ErrorKind::Network
            | boltz_client::util::error::ErrorKind::BoltzApi => {
                SwapError::ServersUnreachable { err: err.message }
            }
            boltz_client::util::error::ErrorKind::Input => SwapError::BadResponse,
            _ => SwapError::BoltzGeneric { err: err.message },
        }
    }
}

pub struct WalletInfo {
    pub balance_sat: u64,
    pub pubkey: String,
    pub active_address: String,
}

pub struct OngoingSwap {
    pub id: String,
    pub preimage: String,
    pub redeem_script: String,
    pub blinding_key: String,
}