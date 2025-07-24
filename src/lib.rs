//! # Verdyce Core
//!
//! A time-decay threshold consensus engine for decentralized voting and governance.
//!
//! ## Features
//!
//! - **Time-decay voting**: Vote weights decrease over time to encourage early participation
//! - **Threshold escalation**: Approval thresholds increase over time for higher scrutiny
//! - **Smart voting windows**: Configurable duration with grace periods and auto-extension
//! - **Multiple decay models**: Linear, exponential, and stepped decay functions
//! - **Flexible thresholds**: Linear, exponential, and sigmoid threshold progression
//!
//! ## Usage
//!
//! ```rust
//! use verdyce_core::engine::Engine;
//! use verdyce_core::models::proposal::Proposal;
//! use verdyce_core::decay::DecayModel;
//! use verdyce_core::threshold::ThresholdModel;
//!
//! let mut engine = Engine::new();
//! let proposal = Proposal::new(
//!     "Test Proposal".to_string(),
//!     "A test proposal".to_string(),
//!     3600, // 1 hour duration
//!     DecayModel::Linear,
//!     ThresholdModel::Linear(0.1, 0.5)
//! );
//! engine.add_proposal(proposal);
//! ```

pub mod decay;
pub mod engine;
pub mod models;
pub mod threshold;
pub mod window;
