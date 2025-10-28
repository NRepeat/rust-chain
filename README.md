# Rust-Chain: A Proof-of-Stake Blockchain

This project is a simplified implementation of a Proof-of-Stake (PoS) blockchain in Rust. It is built with a focus on Clean Architecture principles and demonstrates the core concepts of a distributed ledger, including transaction processing, consensus, and peer-to-peer communication.

## Features

- **Proof-of-Stake (PoS) Consensus:** A simple, round-robin PoS consensus mechanism for selecting block proposers.
- **Clean Architecture:** A clear separation of concerns between domain logic, application use cases, and infrastructure.
- **RESTful API:** An Axum-based API for interacting with the blockchain, including endpoints for creating transactions, viewing blocks, and checking balances.
- **Peer-to-Peer Communication:** Basic P2P functionality for node discovery, chain synchronization, and broadcasting of blocks and votes.
- **In-Memory Storage:** Utilizes in-memory data structures for storing the blockchain, user state, and mempool.
- **HMAC-based Block Signing:** Blocks are signed and verified using HMAC-SHA256 to ensure integrity.

## Architecture

The project follows the principles of Clean Architecture, which results in a modular, testable, and maintainable codebase. The code is organized into four main layers:

- **Domain (`src/domain`):** The core of the application, containing the business logic and entities such as `Block`, `Transaction`, and `Node`. It also defines the repository interfaces (`BlockchainRepository`, `MempoolRepository`, `UserStateRepository`) that are implemented by the infrastructure layer.
- **Application (`src/blockchain/use_cases`):** This layer orchestrates the flow of data and executes the business logic defined in the domain layer. It contains the use cases, such as `create_new_block`, `pos_consensus_loop`, and `sync_chain_task`.
- **Infrastructure (`src/infrastructure`):** This layer contains the concrete implementations of the repository interfaces defined in the domain layer. In this project, we use in-memory repositories.
- **API (`src/api`):** The outermost layer, which exposes the application's functionality to the outside world through a RESTful API. It handles HTTP requests and responses, and it depends on the application layer to perform the requested actions.

## Core Concepts

- **Block:** The fundamental building block of the chain. Each block contains a header, a list of transactions, its own hash, and the hash of the previous block.
- **Transaction:** A transfer of value from a sender to a receiver.
- **User State:** A key-value store that maps user UUIDs to their account balances.
- **Mempool:** A temporary storage for transactions that have been submitted but not yet included in a block.
- **Node:** A participant in the network that maintains a copy of the blockchain and communicates with other nodes.
- **Vote:** A message broadcast by a validator to signal its approval of a proposed block.

## Proof-of-Stake (PoS) Consensus

The consensus mechanism is a simplified, round-robin Proof-of-Stake implementation:

1.  **Slot-based Progression:** Time is divided into slots (e.g., 5 seconds).
2.  **Leader Selection:** In each slot, a leader is chosen from a predefined list of validators in a round-robin fashion.
3.  **Block Proposal:** The leader for the current slot is responsible for creating a new block from the transactions in the mempool and broadcasting it to its peers.
4.  **Block Validation & Voting:** When a validator receives a new block, it verifies the block's integrity, signature, and transactions. If the block is valid, the validator broadcasts an "ACK" vote for that block.
5.  **Quorum & Finalization:** A block is considered finalized once it has received a quorum of votes (more than half of the validators). The leader who proposed the block is then responsible for adding it to their chain.
6.  **Chain Synchronization:** If a node finds that its chain is shorter than a peer's chain, it will initiate a synchronization process to download the longer chain.

## API Endpoints

| Method | Path                  | Description                               |
| ------ | --------------------- | ----------------------------------------- |
| GET    | `/blocks`             | Get all blocks in the chain.              |
| GET    | `/transactions`       | Get all transactions in the mempool.      |
| POST   | `/transactions`       | Create a new transaction.                 |
| POST   | `/user`               | Create a new user with an initial balance.|
| GET    | `/balances`           | Get the balances of all users.            |
| GET    | `/balance/:address`   | Get the balance of a specific user.       |
| POST   | `/block`              | Receive a new block from a peer.          |
| POST   | `/vote`               | Receive a vote from a peer.               |

## Configuration

The application is configured using a `.env` file in the root of the project.

```
SHARED_KEY="your-secret-key"
GENESIS_SENDER_ID="00000000-0000-0000-0000-000000000000"
FAUCET_WALLET_ID="11111111-1111-1111-1111-111111111111"
```

- **`SHARED_KEY`:** A secret key used for signing and verifying blocks with HMAC-SHA256.
- **`GENESIS_SENDER_ID`:** The UUID of the "system" user that funds the faucet in the genesis block.
- **`FAUCET_WALLET_ID`:** The UUID of the faucet wallet, which is used to fund new users.

## How to Run

### Single Node

1.  **Install Rust:** If you don't have Rust installed, you can install it from [rust-lang.org](https://www.rust-lang.org/).
2.  **Create a `.env` file:** Create a `.env` file in the root of the project and add the configuration variables as described above.
3.  **Run the node:**

    ```sh
    cargo run -- --id v1 --port 3001 --peers 3002,3003
    ```

### Multi-Node Network

To run a multi-node network, you can open multiple terminal windows and run each node with a different ID, port, and list of peers.

**Node 1:**

```sh
cargo run -- --id v1 --port 3001 --peers 3002,3003
```

**Node 2:**

```sh
cargo run -- --id v2 --port 3002 --peers 3001,3003
```

**Node 3:**

```sh
cargo run -- --id v3 --port 3003 --peers 3001,3002
```

## Project Structure

```
/
├── src/
│   ├── api/                # Axum web server, handlers, and DTOs
│   ├── blockchain/         # Application layer use cases
│   ├── domain/             # Core business logic and entities
│   └── infrastructure/     # Concrete implementations of repositories
├── .env                    # Configuration file
├── Cargo.toml              # Project dependencies
└── README.md               # This file
```