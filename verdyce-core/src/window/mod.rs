use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowState {
    NotStarted,
    Open,
    Extended,
    GracePeriod,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingWindow {
    pub start_time: DateTime<Utc>,
    pub duration: u64,     
    pub grace_period: u64, 
    pub extended_by: u64
}

impl VotingWindow {
    pub fn new(start_time: DateTime<Utc>, duration: u64, grace_period: u64) -> Self {
        Self {
            start_time,
            duration,
            grace_period,
            extended_by: 0,
        }
    }

    pub fn elapsed(&self, now: DateTime<Utc>) -> u64 {
        (now - self.start_time).num_seconds().max(0) as u64
    }

    pub fn total_duration(&self) -> u64 {
        self.duration + self.extended_by
    }

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

    pub fn extend(&mut self, seconds: u64) {
        self.extended_by += seconds;
    }

    pub fn phase(&self, now: DateTime<Utc>) -> u8 {
        let elapsed = self.elapsed(now);
        let total = self.total_duration();
        if elapsed <= total / 3 {
            1
        } else if elapsed <= (2 * total) / 3 {
            2
        } else {
            3
        }
    }
}
