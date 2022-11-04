cs_test
=======

Test exercise for [ChainSafe](https://chainsafe.io/).

The application uploads a file to IPFS and stores its CID in a smart contract.




## Usage

You need running IPFS and Ethereum node (tested with **ganache**).

Also you need `solc` (Solidity compiler) being installed.

Then:

```
$ cargo run -- --help
Test exercise for ChainSafe

Usage: cs_test [OPTIONS] --file <PATH> --eth-api-address <ETH_API_ADDRESS> --mnemonic <MNEMONIC>

Options:
  -f, --file <PATH>
          Set path to the uploaded file
  -e, --eth-api-address <ETH_API_ADDRESS>
          Set Ethereum JSON-RPC API address
  -i, --ipfs-api-address <IPFS_API_ADDRESS>
          Set IPFS API address (for example, http://localhost:5001/api/v0). Will use the one from ~/.ipfs/api or localhost:5001 otherwise
  -m, --mnemonic <MNEMONIC>
          Set mnemonic for wallet, which will be used for contract deploy
  -p, --password <PASSWORD>
          Provide password if the mnemonic is encrypted
  -h, --help
          Print help information
  -V, --version
          Print version information
```
