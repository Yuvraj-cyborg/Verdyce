# Verdyce

[![Verdyce on crates.io](https://img.shields.io/crates/v/verdyce.svg)](https://crates.io/crates/verdyce)
[![Documentation](https://docs.rs/verdyce/badge.svg)](https://docs.rs/verdyce)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A time-decay threshold consensus engine for decentralized voting and governance.

This library provides a pure Rust implementation of consensus mechanisms with configurable decay models and dynamic approval thresholds, allowing developers to build decentralized governance systems that encourage early participation while maintaining security through escalating scrutiny over time.

## Components

### Library

The following modules provide the core functionality:

- [`engine`](src/engine.rs) provides the main consensus coordinator for managing proposals and votes
- [`models`](src/models/) contains the core data structures for proposals and votes
- [`decay`](src/decay/) implements time-decay models for vote weight calculation
- [`threshold`](src/threshold/) provides threshold progression functions
- [`window`](src/window/) manages voting window state and timing

#### Using as a library

Add `verdyce` to your project to access all components:

```toml
[dependencies]
verdyce = "0.1.0"
```

A simple example of use follows. For more details, please visit the [documentation](https://docs.rs/verdyce).

```rust
use verdyce::{
    engine::Engine,
    models::{proposal::Proposal, vote::{Vote, VoteChoice}},
    decay::DecayModel,
    threshold::ThresholdModel,
};
use uuid::Uuid;
use chrono::Utc;

let mut engine = Engine::new();

let proposal = Proposal::new(
    "Protocol Upgrade".to_string(),
    "Upgrade to version 2.0".to_string(),
    3600, // 1 hour voting period
    DecayModel::Linear,
    ThresholdModel::Linear(0.0001, 0.5),
);

let proposal_id = proposal.id;
engine.add_proposal(proposal);

let vote = Vote {
    validator_id: Uuid::new_v4(),
    choice: VoteChoice::Yes,
    timestamp: Utc::now(),
    revision: 0,
    reason: None,
};

engine.cast_vote(proposal_id, vote);
engine.evaluate_all(Utc::now());
```

## Features

### Time-Decay Voting
Vote weights decrease over time to encourage early participation:
- **Linear** - Steady decline from 1.0 to 0.1
- **Exponential** - Rapid early decline with configurable rate  
- **Stepped** - Discrete weight levels across voting phases

### Dynamic Thresholds
Approval thresholds increase over time for higher scrutiny:
- **Linear** - Steady increase: `threshold = t × rate + start`
- **Exponential** - Asymptotic growth with configurable parameters
- **Sigmoid** - S-curve progression for smooth transitions

### Smart Voting Windows
- Configurable duration with grace periods
- Auto-extension when near threshold and time expiry
- State tracking through the proposal lifecycle

### Revision Penalties
Vote changes are penalized to discourage manipulation while allowing legitimate updates.

## Building

You can use Cargo to build the library:

```sh
cargo build
```

Minimum supported Rust version is 1.70.0.

## Testing

Run the test suite:

```sh
cargo test
```

Generate and view documentation:

```sh
cargo doc --open
```

## Contributing

This project is under active development. Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Code is properly formatted: `cargo fmt` 
3. No clippy warnings: `cargo clippy`
4. Documentation is updated for public APIs

See the [contributor guidelines](CONTRIBUTING.md) for more details.

## License

Licensed under the MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be licensed as above, without any additional terms or conditions.