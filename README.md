# UniswapX Atomic Filler

This is a sample [Artemis](https://github.com/paradigmxyz/artemis) bot that fills UniswapX orders atomically using Uniswap v2 and v3 liquidity.

Feel free to fork and modify to run any strategies you wish to fill UniswapX orders!

# Usage

First you must deploy an executor contract that implements the [IReactorCallback](https://github.com/Uniswap/UniswapX/blob/main/src/interfaces/IReactorCallback.sol) interface. This sample currently uses the provided [SwapRouter02Executor](https://github.com/Uniswap/UniswapX/blob/main/src/sample-executors/SwapRouter02Executor.sol).

Then update the address constant in [uniswapx_strategy](./src/strategies/uniswapx_strategy.rs) to point to your executor contract.

Finally, run the bot with the following command:

```
cargo run -- --wss <websocket RPC url> --private-key <private key> --bid-percentage <percent of profit to share as gas>
```

# Collectors

### [block-collector](./src/collectors/block_collector.rs)

Collects new blocks as they are confirmed. Similar to the base one in Artemis-core but includes timestamp data to resolve dutch decays

### [uniswapx-order-collector](./src/collectors/uniswapx_order_collector.rs)

Collects new executable UniswapX orders as they are posted.

### [uniswapx-route-collector](./src/collectors/uniswapx_route_collector.rs)

Finds on-chain AMM routes to fill UniswapX orders. Ran in a separate collector thread as these can be slow and don't want to block other processing.

# Strategies

### [uniswapx-strategy](./src/strategies/uniswapx_strategy.rs)

Simple strategy that batches UniswapX orders together by tokenin/tokenout pair and attempts to fill using Uniswap protocol liquidity

# Crates

### [uniswapx-rs](./crates/uniswapx-rs)
Library for encoding, decoding, and resolving UniswapX dutch orders

### [bindings-uniswapx](./crates/bindings-uniswapx)
Autogenerated forge bindings for UniswapX contracts
