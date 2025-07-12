# 🗳️ Time-Decay Threshold Consensus System

![Architecture Diagram](./architecture.png)

A modular Rust-based consensus engine designed for decentralized voting using **time-weighted votes** and **escalating decision thresholds**.

This system is ideal for scenarios requiring early-vote preference, strategic participation incentives, and robust fault-tolerant governance — like validator-based blockchains, DAOs, or decentralized proposals.

---

## 📜 Features

### ✅ Time-Weighted Voting
- Vote weight **decays** over time to reward early participation.
- Configurable decay models: `Exponential`, `Linear`, `Stepped`.
- Penalties for vote **revisions** to discourage flip-flopping.

### 📈 Dynamic Thresholds
- Thresholds **start low** and increase over time to promote early resolution.
- Supports `Linear`, `Exponential`, and `Sigmoid` threshold models.
- Ceiling/floor bounds for safety (e.g., min 35%, max 90%).

### 🪟 Smart Voting Windows
- Voting windows with:
  - Grace periods
  - Automatic extensions near consensus
  - Voting phases (early/mid/late)
- Window states: `Open`, `Extended`, `GracePeriod`, `Expired`.

### 🎛️ Multi-Phase Architecture (Optional Extension)
- Phase 1: Low threshold, high weight
- Phase 2: Medium threshold/weight
- Phase 3: High threshold, low weight

### 🧮 Core Modules
| Module | Responsibility |
|--------|----------------|
| `vote.rs` | Vote definition, weight calculation with decay & revision |
| `proposal.rs` | Proposal lifecycle, evaluation logic |
| `window.rs` | Voting window timing, extension, phase classification |
| `threshold.rs` | Threshold escalation strategies |
| `engine.rs` | System coordinator: vote casting, evaluation, extension |

---

## 🔧 CLI Usage (Coming Soon)

A command-line interface to interact with proposals and cast votes.

### 🚀 Planned Commands

```bash
# Create a new proposal
vote-cli new --title "Change Param X" --desc "Proposal to update X" --duration 300 \
    --decay Exponential:0.1 \
    --threshold Linear:0.01,0.5

# Cast a vote on a proposal
vote-cli cast --proposal-id <UUID> --choice Yes --validator-id <UUID>

# Evaluate all proposals
vote-cli evaluate-all

# Extend voting windows where applicable
vote-cli extend-all

# Check system status
vote-cli status
```

---

## 🛠️ Getting Started

### 1. 🧱 Build the Project
```bash
cargo build
```

### 2. ✅ Run the Tests
```bash
cargo test
```

### 3. 🔬 Example Evaluation
You can simulate proposal evaluation with fake timestamps:

```rust
use chrono::Utc;
engine.evaluate_all(Utc::now());
```

---

## 🧠 Vote Weight Decay Explained

| Model | Description | Example |
|-------|-------------|---------|
| Linear | Weight drops linearly from 1.0 → 0.1 | 50% time = 0.55 |
| Exponential | Drops sharply early on | 10% time = ~0.9, 90% = ~0.1 |
| Stepped | Discrete weight levels across phases | Phase 1 = 1.0, Phase 3 = 0.1 |

---

## 📊 Threshold Progression Models

| Model | Formula | Notes |
|-------|---------|-------|
| Linear | threshold = t * r + s | Ramps from base to cap |
| Exponential | threshold = s + (1 - e^(-r * t)) | Quick early rise |
| Sigmoid | Smooth non-linear curve | Good for adaptive ramping |

---

## 🧱 Architecture Overview

All proposals are managed via the `Engine` struct.

Evaluation is done by calculating:
- **Approval ratio**: `yes_weight / total_weight`
- **Threshold**: via time-progression function

Proposals move from `Pending → Accepted/Rejected/Expired`

---

## 📂 Project Structure

```
src/
├── main.rs          # (optional CLI binary)
├── engine.rs        # Proposal coordinator
├── models/
│   ├── proposal.rs  # Proposal definition and evaluation logic
│   ├── vote.rs      # Vote structure and weighted evaluation
├── decay/
│   └── mod.rs       # Decay model logic
├── threshold/
│   └── mod.rs       # Threshold progression engine
├── window/
│   └── mod.rs       # VotingWindow logic, state & phase transitions
├── reputation/
│   └── mod.rs       # Reputation system (future)
```

---

## ⚙️ Configuration Examples

You can plug in different configurations at runtime:

```rust
use verdyce_core::models::*;

// Configure decay model
let decay = DecayModel::Exponential(0.1);

// Configure threshold progression
let threshold = ThresholdModel::Linear(0.01, 0.51); // 1% increase per second, starts at 51%

// Configure voting window
let window = VotingWindow::new(Utc::now(), 120, 30); // 2-min voting, 30-sec grace
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request