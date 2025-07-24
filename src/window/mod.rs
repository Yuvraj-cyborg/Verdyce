//! # Voting Window Management
//!
//! Manages the timing and state of voting periods, including extensions and grace periods.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents the current state of a voting window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowState {
    /// Voting has not yet started
    NotStarted,
    /// Voting is currently open and accepting votes
    Open,
    /// Voting window has been extended and is still accepting votes
    Extended,
    /// In grace period after voting ended (final evaluation time)
    GracePeriod,
    /// Window has completely expired
    Expired,
}

/// Represents the current phase within an active voting period.
///
/// Used for future features and analytics.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VotingPhase {
    /// First third of the voting period
    Early,
    /// Middle third of the voting period  
    Mid,
    /// Final third of the voting period
    Late,
}

/// Manages the timing and state of a voting window.
///
/// A voting window consists of:
/// - Main voting period (duration)
/// - Optional extensions (extended_by)
/// - Grace period for final evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingWindow {
    /// When voting begins
    pub start_time: DateTime<Utc>,
    /// Base duration of the voting period in seconds
    pub duration: u64,
    /// Grace period after voting ends for final evaluation
    pub grace_period: u64,
    /// Additional time added through extensions
    pub extended_by: u64,
}

impl VotingWindow {
    /// Creates a new voting window.
    ///
    /// # Arguments
    /// * `start_time` - When voting begins
    /// * `duration` - Base voting period duration in seconds
    /// * `grace_period` - Grace period after voting ends in seconds
    pub fn new(start_time: DateTime<Utc>, duration: u64, grace_period: u64) -> Self {
        Self {
            start_time,
            duration,
            grace_period,
            extended_by: 0,
        }
    }

    /// Calculates how much time has elapsed since voting started.
    ///
    /// # Arguments
    /// * `now` - Current timestamp
    ///
    /// # Returns
    /// Elapsed time in seconds (0 if voting hasn't started yet)
    pub fn elapsed(&self, now: DateTime<Utc>) -> u64 {
        (now - self.start_time).num_seconds().max(0) as u64
    }

    /// Returns the total duration including any extensions.
    pub fn total_duration(&self) -> u64 {
        self.duration + self.extended_by
    }

    /// Determines the current state of the voting window.
    ///
    /// # Arguments
    /// * `now` - Current timestamp
    ///
    /// # Returns
    /// The current window state
    pub fn state(&self, now: DateTime<Utc>) -> WindowState {
        let elapsed = self.elapsed(now);

        if now < self.start_time {
            WindowState::NotStarted
        } else if elapsed <= self.total_duration() {
            if self.extended_by > 0 {
                WindowState::Extended
            } else {
                WindowState::Open
            }
        } else if elapsed <= self.total_duration() + self.grace_period {
            WindowState::GracePeriod
        } else {
            WindowState::Expired
        }
    }

    /// Extends the voting window by the specified number of seconds.
    ///
    /// # Arguments
    /// * `seconds` - Number of seconds to add to the voting period
    pub fn extend(&mut self, seconds: u64) {
        self.extended_by += seconds;
    }

    /// Determines which phase of voting we're currently in.
    ///
    /// # Arguments
    /// * `now` - Current timestamp
    ///
    /// # Returns
    /// The current voting phase (Early, Mid, or Late)
    pub fn phase(&self, now: DateTime<Utc>) -> VotingPhase {
        let elapsed = self.elapsed(now);
        let total = self.total_duration();

        if elapsed <= total / 3 {
            VotingPhase::Early
        } else if elapsed <= (2 * total) / 3 {
            VotingPhase::Mid
        } else {
            VotingPhase::Late
        }
    }
}
