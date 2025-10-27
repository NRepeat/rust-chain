# Simple Blockchain in Rust

This project is a simple implementation of a blockchain in Rust, built following Clean Architecture principles. It includes basic functionalities like creating a blockchain, adding blocks, managing a mempool of transactions, and processing transactions based on account balances.

## Architecture

The project is structured following the principles of **Clean Architecture**. This means the code is organized in layers, with the domain logic at the center and dependencies pointing inwards.

-   **Domain Layer (`src/domain`):** Contains the core business logic and entities of the application. This includes `Block`, `Transaction`, `Mempool`, and the repository traits (`BlockchainRepository`, `MempoolRepository`, `StateRepository`). This layer has no dependencies on other layers.
-   **Application/Use Case Layer (`src/blockchain`):** Orchestrates the flow of data and calls the domain layer to perform business logic. This layer contains the use cases like `create_genesis_block`, `create_new_block`, and `process_mempool`.
-   **Infrastructure Layer (`src/infrastructure`):** Contains the concrete implementations of the repository traits. In this project, we use in-memory repositories (`InMemoryBlockchainRepository`, `InMemoryMempoolRepository`, `InMemoryStateRepository`). This layer depends on the domain layer.
-   **Entry Point (`src/main.rs`):** The main executable that composes the application by instantiating the concrete implementations and injecting them into the use cases.

## Core Concepts

### Block

A `Block` is the fundamental building block of the blockchain. It contains:
-   An index (its position in the chain)
-   A timestamp
-   A list of transactions
-   The hash of the previous block
-   Its own hash
-   A nonce used for mining

### Transaction

A `Transaction` represents the transfer of value from one account to another. It includes:
-   A unique ID
-   The sender's address (`from`)
-   The receiver's address (`to`)
-   The amount
-   A timestamp

### Mempool

The `Mempool` is a waiting area for transactions that have been submitted but not yet included in a block. It holds a queue of transactions and provides methods to add and drain them.

### State

The `State` of the blockchain represents the current balances of all accounts. It is managed by a `StateRepository` and is used to validate transactions before they are included in a block.

## Repositories

The project uses the repository pattern to decouple the domain logic from the data storage implementation.

-   **`BlockchainRepository`**: Defines the interface for storing and retrieving blocks.
-   **`MempoolRepository`**: Defines the interface for managing the transaction mempool.
-   **`StateRepository`**: Defines the interface for managing account balances.

## Use Cases

-   **`create_genesis_block`**: Creates the first block in the chain.
-   **`create_new_block`**: Mines a new block with a given set of transactions and adds it to the chain.
-   **`validate_chain`**: Checks the integrity of the entire blockchain.
-   **`process_mempool`**: Processes the transactions in the mempool, validates them against the current state (balances), and returns the valid transactions to be included in the next block.

## How to Run

To run the project, you need to have Rust installed. You can then run the following command in the project's root directory:

```sh
cargo run
```

This will execute the `main` function in `src/main.rs`, which demonstrates the process of creating a blockchain, processing transactions, and adding a new block.

## Refactoring and Implementation Steps

This project has been developed through a series of refactoring and implementation steps to adhere to Clean Architecture and add new features.

1.  **Initial Refactoring**: The `Mempool` was refactored to remove direct dependencies on the infrastructure layer. A `MempoolRepository` trait was introduced in the domain layer, and dependency injection was used to provide the concrete implementation.
2.  **Moving `BlockchainRepository`**: The `BlockchainRepository` trait was moved from the application layer to the domain layer to maintain consistency with Clean Architecture principles.
3.  **State Management**: A `StateRepository` was introduced to manage account balances. An in-memory implementation was created to store balances in a `HashMap`.
4.  **Mempool Processing**: The `process_mempool` use case was created to process transactions from the mempool. This use case iterates through the transactions, checks the sender's balance using the `StateRepository`, and applies the transaction if the balance is sufficient.
5.  **Integration**: The new functionality was integrated into the `main` function to demonstrate the end-to-end flow of processing transactions and creating a new block.
