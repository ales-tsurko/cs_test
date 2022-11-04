//! IPFS interface

use std::fs::File;
use std::path::PathBuf;

use http::Uri;
use ipfs_api_backend_actix::{IpfsApi, IpfsClient, TryFromUri};

/// Send a file to IPFS network
pub async fn send_file(path: &PathBuf, ipfs_api_address: Option<Uri>) -> anyhow::Result<String> {
    // IpfsClient::from_str uses unwrap in its implementation. This causes the problem related to
    // http::Uri. For example, Uri successfuly parses `foo` and splits it into parts. But when we
    // try to build it up back from the parts, it returns an error (because of not having scheme, in
    // this particular case). We don't have this problem when we use http::Uri directly, because
    // IpfsClient discovers that the URI isn't ok later, when we start calling API. This is not
    // good, because we omit early return. But I considered it as a fine solution for a test
    // exercise.
    let ipfs_client =
        ipfs_api_address.map_or(IpfsClient::default(), IpfsClient::build_with_base_uri);

    let file = File::open(path)?;
    match ipfs_client.add(file).await {
        Ok(res) => Ok(res.hash),
        Err(e) => Err(anyhow::anyhow!(format!("error adding file: {}", e))),
    }
}
