<div align="center">
   <img src="./.images/ink-logo-glow.svg" alt="ink!" height="136" />
</div>

This repository contains a set of example contracts for ink!.

Have a look at the different examples to better understand how to use ink! to build your own Substrate smart contracts.

### Can I add a new example here?

Please don't add them here, but create a Pull Request to the `integration-tests` folder in [the ink! repository](https://github.com/paritytech/ink) instead.
The content of that folder is synchronized with this repository on new releases.

## Preparation

For building the example smart contracts found in this folder you will need to have [`cargo-contract`](https://github.com/paritytech/cargo-contract) installed.

```
cargo install cargo-contract --force
```

We use the `--force` to update to the most recent `cargo-contract` version.

## Build example contract and generate the contracts metadata

To build a single example and generate the contracts Wasm file, navigate to the root of the smart contract and run the following command:

`cargo contract build`

You should now have an optimized `<contract-name>.wasm` file, a `metadata.json` file and a `<contract-name>.contract` file in the `target` folder of your contract.
The `.contract` file combines the Wasm and metadata into one file and can be used for instantiation.

## License

The examples in this folder are released into the public domain.
We hope they help you build something great with ink!.

See the [LICENSE file](LICENSE) in this folder for more details.
