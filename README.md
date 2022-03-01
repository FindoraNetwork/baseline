# Baseline

A asynchronous blockchain development framework in rust.

## Features

- Modularized
- Upgradeable
- Parallelized
- Compat multi consensus
- Customizable
- Unitest support

## Usage

### Quick start

Generate crate using `cargo generate`

``` shell
$ cargo generate tiannian/baseline-template
```

Then set crate name, author and `level-type`.

The `level-type` include three choices: `module`, `manager`, `runtime`. Refer to [Level Type](#).

## Design Principles

For overall design architecture and understanding, see reference.

## Framework Level Type

Baseline contains three level concepts, module, manager and runtime.

### Module

Each module contains some components.

- Context
    - Storage backend
    - Hasher
    - Manager info.
- Storage
    - Genesis Value
    - Upgrade compatible
    - Merkle
- Consensus (In future)
    - Chain selection
    - Proposer selection
    - Vote
- Block
    - Apply transaction
    - Process byzantine punish
    - Manage validator set
    - Do state sync
- Mempool
    - Check module transaction
    - Convert transaction format
    - Apply pending state.
- RPC
    - Storage access
    - User defined RPC
- Dependence
- Version
    - Upgrade based on height. (hard fork).
    - Upgrade based on implementation (soft fork).
- Event

### Manager

- Execution
    - Order
- Block
    - Commit and drop storage
    - Storage transaction and block
- Consensus
    - Validator set
- Mempool
    - Deserialize and verify transaction
- RPC
    - Module
    - Event and filter
    - Transaction
    - Block
    - Node and P2P
- P2P
    - Node key
    - Connection
    - Node discover
- Dependence
   - Injection

### Runtime

- Consensus
- P2P
- RPC
- Utils
