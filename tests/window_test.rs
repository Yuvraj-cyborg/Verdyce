use chrono::{Duration, Utc};
use verdyce_core::window::{VotingWindow, WindowState};

#[test]
fn test_not_started_state() {
    let future_time = Utc::now() + Duration::seconds(60);
    let window = VotingWindow::new(future_time, 120, 30);
    assert_eq!(window.state(Utc::now()), WindowState::NotStarted);
}

#[test]
fn test_open_state() {
    let start = Utc::now() - Duration::seconds(10);
    let window = VotingWindow::new(start, 60, 30);
    assert_eq!(window.state(Utc::now()), WindowState::Open);
}

#[test]
fn test_expired_state() {
    let start = Utc::now() - Duration::seconds(130);
    let window = VotingWindow::new(start, 60, 50);
    assert_eq!(window.state(Utc::now()), WindowState::Expired);
}
