use cs_test::cli::parse_cli;
use cs_test::eth::Contract;
use cs_test::ipfs::send_file;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let cli = parse_cli();

    let cid = send_file(&cli.file, cli.ipfs_api_address.clone()).await?;

    let contract = Contract::new(cid, cli.eth_api_address, cli.mnemonic, cli.password);

    let cid = contract.evaluate().await?;

    println!("CID stored in contract {}", cid);

    Ok(())
}
