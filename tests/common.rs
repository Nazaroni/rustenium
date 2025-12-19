use rustenium::common::wait::wait_for;
use std::time::Duration;

#[tokio::test]
async fn test_wait_for_true() {
    let result = wait_for(Duration::from_millis(500), || async { true }).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_wait_for_timeout() {
    let result = wait_for(Duration::from_millis(500), || async { false }).await;
    assert!(result.is_err());
}
