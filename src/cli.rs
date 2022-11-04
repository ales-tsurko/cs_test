//! Command Line Interface

use std::path::PathBuf;

use clap::Parser;
use http::uri::Uri;

/// Parse CLI arguments and options
pub fn parse_cli() -> Cli {
    Cli::parse()
}

/// Command Line Interface parser.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Set path to the uploaded file
    #[arg(short, long, value_name = "PATH")]
    pub file: PathBuf,

    /// Set Ethereum JSON-RPC API address
    #[arg(short, long, value_name = "ETH_API_ADDRESS")]
    pub eth_api_address: String,

    /// Set IPFS API address (for example, http://localhost:5001/api/v0). Will use the one from
    /// ~/.ipfs/api or localhost:5001 otherwise.
    #[arg(short, long, value_name = "IPFS_API_ADDRESS")]
    pub ipfs_api_address: Option<Uri>,

    /// Set mnemonic for wallet, which will be used for contract deploy
    #[arg(short, long, value_name = "MNEMONIC")]
    pub mnemonic: String,

    /// Provide password if the mnemonic is encrypted
    #[arg(short, long, value_name = "PASSWORD")]
    pub password: Option<String>,
}
