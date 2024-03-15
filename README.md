# Stylus workshop: Counter program

Simple Counter program written in Rust.

## Getting started

Follow the instructions in the [Stylus quickstart](https://docs.arbitrum.io/stylus/stylus-quickstart) to configure your development environment.

You'll also need [Foundry](https://github.com/foundry-rs/foundry) to build and deploy the ERC-20 contract.

## Testing the contract

The `scripts` folder contains several scripts that make individual calls to perform the most important actions:

1. [./scripts/deploy.sh](./scripts/deploy.sh) to deploy the contract
2. [./scripts/getCount.sh](./scripts/getCount.sh) to get the current counter
3. [./scripts/increment.sh](./scripts/increment.sh) to increment the current counter

## How to run a local Stylus dev node

Instructions to setup a local Stylus dev node can be found [here](https://docs.arbitrum.io/stylus/how-tos/local-stylus-dev-node).

## How to get ETH for the Stylus testnet

The Stylus testnet is an L3 chain that settles to Arbitrum Sepolia. The usual way of obtaining ETH is to bridge it from Arbitrum Sepolia through the [Arbitrum Bridge](https://bridge.arbitrum.io/?destinationChain=stylus-testnet&sourceChain=arbitrum-sepolia). You can find a list of Arbitrum Sepolia faucets [here](https://docs.arbitrum.io/stylus/reference/testnet-information#faucets).

## Useful resources

- [Stylus quickstart](https://docs.arbitrum.io/stylus/stylus-quickstart)
- [Stylus by example](https://arbitrum-stylus-by-example.vercel.app/)
- [Awesome Stylus](https://github.com/OffchainLabs/awesome-stylus)
- [Counter program](https://github.com/OffchainLabs/stylus-workshop-counter)
- [Interactions between Rust and Solidity](https://github.com/OffchainLabs/stylus-workshop-rust-solidity/)
- [Workshop presentation]()
- [Telegram group](https://t.me/arbitrum_stylus)
- [Discord channel](https://discord.com/channels/585084330037084172/1146789176939909251)

## Stylus reference links

- [Stylus documentation](https://docs.arbitrum.io/stylus/stylus-gentle-introduction)
- [Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs)
- [Cargo Stylus](https://github.com/OffchainLabs/cargo-stylus)