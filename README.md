# ANDE Reth - Parallel EVM & Dual Token + K'intu Living Genome

This is a fork of `ev-reth` customized for AndeChain, with three major innovations:
1. **Parallel EVM** for increased throughput
2. **Dual Token Support** inspired by Celo
3. **K'intu Living Genome** - Real genomic data preserved on-chain

> **See [GENESIS.md](./GENESIS.md) for sacred plants genomic data and discovery mechanism**

## Key Modifications

This section details the primary changes made to the original `ev-reth` codebase.

### 1. Parallel EVM

To significantly improve transaction processing speed, we've integrated a parallel execution model. This allows Reth to process multiple transactions concurrently, leveraging multi-core architectures.

-   **What was changed?**
    -   We introduced a parallel transaction executor, a multi-version memory data structure (`MvMemory`), and a scheduler to manage concurrent execution.
-   **Where are the changes?**
    -   The core logic is located in `crates/evolve/src/parallel/`.
        -   `executor.rs`: Manages the concurrent execution of transactions.
        -   `mv_memory.rs`: Implements a multi-version memory model to handle state conflicts.
        -   `scheduler.rs`: Coordinates the scheduling and execution of transaction processing tasks.
-   **Why was it changed?**
    -   To increase the transaction throughput of the node, allowing it to handle a higher volume of transactions per second.
-   **How does it work?**
    1.  The scheduler receives a block of transactions.
    2.  It distributes the transactions to multiple worker threads.
    3.  Each thread executes its assigned transactions in parallel, using `MvMemory` to read from a shared state and write to a local cache.
    4.  After execution, the scheduler identifies and resolves any conflicts, re-executing transactions sequentially if necessary.
    5.  Finally, it commits the results to the main state.

### 2. Dual Token Support (ANDE Token Duality)

We've implemented a dual-token system, similar to Celo, to separate the token used for gas fees from the token used for staking and governance.

-   **What was changed?**
    -   We integrated a precompile for native balance transfers and created a custom EVM configuration.
-   **Where are the changes?**
    -   The main components are in `crates/evolve/src/evm_config/`.
        -   `precompile.rs`: The precompile at `0x00...00fd` that handles native token transfers.
        -   `ande_precompile_provider.rs`: Implements the logic for native balance transfers using the `journal.transfer()` API.
        -   `factory.rs`: Creates the `AndeEvmConfig`, which is a type alias for `EthEvmConfig` that includes the ANDE precompile.
-   **Why was it changed?**
    -   To create a more flexible economic model where gas prices can remain stable and low, while the value of the staking/governance token can fluctuate freely.
-   **How does it work?**
    1.  A transaction is initiated to transfer the native token.
    2.  The transaction calls the precompile at address `0x00...00fd`.
    3.  The precompile executes the `AndePrecompileProvider`, which directly calls the `journal.transfer()` function.
    4.  This function modifies the native token balance in the state, bypassing the standard EVM token transfer mechanism for greater efficiency.

## Getting Started

### Prerequisites

-   Rust (latest stable version)
-   Git

### Build and Test

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/andelabs/ev-reth.git
    cd ev-reth
    ```

2.  **Build the project:**

    ```bash
    cargo build --release
    ```

3.  **Run the tests:**

    ```bash
    cargo test
    ```

## Quick Start

### 1. Initialize Genesis
```bash
./target/release/ev-reth init \
    --chain specs/genesis.json \
    --datadir ~/.andechain
```

### 2. Run Node
```bash
./target/release/ev-reth node \
    --chain specs/genesis.json \
    --datadir ~/.andechain \
    --http \
    --http.api all \
    --ws \
    --ws.api all
```

### 3. Verify Genesis
```bash
./target/release/ev-reth dump-genesis --chain specs/genesis.json
```

For debugging, you can enable trace-level logs:
```bash
RUST_LOG=debug,ev-reth=trace ./target/release/ev-reth node
```

## Project Structure

```
ande-reth/
├── specs/              # Chain specifications
│   └── genesis.json   # AndeChain genesis with K'intu embedded
├── contracts/          # Smart contracts
│   └── PlantGenome.sol # Living genome system
├── data/              # Scientific data
│   └── kintu/         # NCBI genomic data
└── crates/            # Rust implementation
    ├── evolve/        # Parallel EVM + Consensus
    └── node/          # Node configuration
```

See [GENESIS.md](./GENESIS.md) for K'intu sacred plants information.